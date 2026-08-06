//! Governed capability, alias, and deprecation taxonomy loading.

use std::{collections::BTreeMap, fs::File, io::Read, path::Path};

use ossus_core::{CapabilityId, CapabilityStatus, Version, validate_schema_version};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use toml::Value;

use crate::{
    budget::{Budget, decode_utf8, parse_bounded_toml},
    diagnostic::{Diagnostic, DiagnosticClass, bounded_value, sort_diagnostics},
};

const BUILTIN_CAPABILITIES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/capabilities-v1.toml"
));
const BUILTIN_ALIASES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/aliases-v1.toml"
));
const BUILTIN_DEPRECATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/deprecations-v1.toml"
));

/// One governed capability declaration as written in the taxonomy TOML.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDefinition {
    pub id: CapabilityId,
    pub domain: String,
    pub name: String,
    pub definition: String,
    pub status: CapabilityStatus,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityDocument {
    schema_version: Version,
    capabilities: Vec<CapabilityDefinition>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AliasDocument {
    schema_version: Version,
    aliases: BTreeMap<String, CapabilityId>,
}

/// Deprecation policy metadata governed alongside the taxonomy.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationPolicy {
    pub minimum_alias_lifetime_major_versions: u16,
    pub removal_requires: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeprecationDocument {
    schema_version: Version,
    deprecations: Vec<CapabilityId>,
    policy: DeprecationPolicy,
}

/// Validated, deterministic taxonomy lookup state.
#[derive(Clone, Debug)]
pub struct Taxonomy {
    schema_version: Version,
    capabilities: BTreeMap<CapabilityId, CapabilityDefinition>,
    aliases: BTreeMap<String, CapabilityId>,
    deprecations: Vec<CapabilityId>,
    policy: DeprecationPolicy,
}

impl Taxonomy {
    /// Loads the built-in TOML taxonomy shipped with Ossus.
    pub fn load_builtin(budget: &Budget) -> Result<Self, Vec<Diagnostic>> {
        Self::from_toml_sources(
            BUILTIN_CAPABILITIES,
            BUILTIN_ALIASES,
            BUILTIN_DEPRECATIONS,
            budget,
        )
    }

    /// Loads taxonomy files from explicit paths with the same parsing budgets as manifests.
    pub fn load_from_paths(
        capabilities_path: &Path,
        aliases_path: &Path,
        deprecations_path: &Path,
        budget: &Budget,
    ) -> Result<Self, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let capabilities = match read_taxonomy_source(capabilities_path, budget) {
            Ok(source) => Some(source),
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                None
            }
        };
        let aliases = match read_taxonomy_source(aliases_path, budget) {
            Ok(source) => Some(source),
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                None
            }
        };
        let deprecations = match read_taxonomy_source(deprecations_path, budget) {
            Ok(source) => Some(source),
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                None
            }
        };
        if !diagnostics.is_empty() {
            sort_diagnostics(&mut diagnostics);
            return Err(diagnostics);
        }

        match (capabilities, aliases, deprecations) {
            (Some(capabilities), Some(aliases), Some(deprecations)) => {
                Self::from_toml_sources(&capabilities, &aliases, &deprecations, budget)
            }
            _ => Err(vec![Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.file.read-failed",
                "$",
                "unable to load all taxonomy sources",
            )]),
        }
    }

    /// Loads taxonomy documents directly from their bounded TOML sources.
    pub fn from_toml_sources(
        capabilities_source: &str,
        aliases_source: &str,
        deprecations_source: &str,
        budget: &Budget,
    ) -> Result<Self, Vec<Diagnostic>> {
        let capabilities_value =
            parse_bounded_toml(capabilities_source, budget, DiagnosticClass::Taxonomy);
        let aliases_value = parse_bounded_toml(aliases_source, budget, DiagnosticClass::Taxonomy);
        let deprecations_value =
            parse_bounded_toml(deprecations_source, budget, DiagnosticClass::Taxonomy);

        let mut diagnostics = Vec::new();
        let capabilities_value = match capabilities_value {
            Ok(value) => Some(value),
            Err(mut errors) => {
                diagnostics.append(&mut errors);
                None
            }
        };
        let aliases_value = match aliases_value {
            Ok(value) => Some(value),
            Err(mut errors) => {
                diagnostics.append(&mut errors);
                None
            }
        };
        let deprecations_value = match deprecations_value {
            Ok(value) => Some(value),
            Err(mut errors) => {
                diagnostics.append(&mut errors);
                None
            }
        };
        if !diagnostics.is_empty() {
            sort_diagnostics(&mut diagnostics);
            return Err(diagnostics);
        }

        let Some(capabilities_value) = capabilities_value else {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.shape.invalid",
                "$",
                "capability taxonomy did not produce a TOML value",
            )]);
        };
        let Some(aliases_value) = aliases_value else {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.shape.invalid",
                "$",
                "alias taxonomy did not produce a TOML value",
            )]);
        };
        let Some(deprecations_value) = deprecations_value else {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.shape.invalid",
                "$",
                "deprecation taxonomy did not produce a TOML value",
            )]);
        };

        let capabilities = decode_document::<CapabilityDocument>(
            capabilities_value,
            "capabilities",
            &mut diagnostics,
        );
        let aliases = decode_document::<AliasDocument>(aliases_value, "aliases", &mut diagnostics);
        let deprecations = decode_document::<DeprecationDocument>(
            deprecations_value,
            "deprecations",
            &mut diagnostics,
        );
        if !diagnostics.is_empty() {
            sort_diagnostics(&mut diagnostics);
            return Err(diagnostics);
        }

        let (Some(capabilities), Some(aliases), Some(deprecations)) =
            (capabilities, aliases, deprecations)
        else {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.shape.invalid",
                "$",
                "taxonomy documents could not be decoded",
            )]);
        };

        Self::from_documents(capabilities, aliases, deprecations)
    }

    fn from_documents(
        capabilities: CapabilityDocument,
        aliases: AliasDocument,
        deprecations: DeprecationDocument,
    ) -> Result<Self, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        if let Err(error) = validate_schema_version(&capabilities.schema_version) {
            diagnostics.push(Diagnostic::from_contract_error(
                DiagnosticClass::Taxonomy,
                "schema_version",
                &error,
            ));
        }
        if capabilities.schema_version.major() != aliases.schema_version.major() {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.schema-version.mismatch",
                "aliases.schema_version",
                "taxonomy documents use different schema major versions",
            ));
        }
        if capabilities.schema_version.major() != deprecations.schema_version.major() {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.schema-version.mismatch",
                "deprecations.schema_version",
                "taxonomy documents use different schema major versions",
            ));
        }

        let mut capability_map = BTreeMap::new();
        for (index, capability) in capabilities.capabilities.iter().enumerate() {
            let expected = format!("{}.{}", capability.domain, capability.name);
            if capability.id.as_str() != expected {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.id.domain-name-mismatch",
                    format!("capabilities[{index}].id"),
                    format!("capability id must equal `{}`", bounded_value(&expected)),
                ));
            }
            if capability_map.contains_key(&capability.id) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.id.duplicate",
                    format!("capabilities[{index}].id"),
                    "capability id is declared more than once",
                ));
            } else {
                capability_map.insert(capability.id.clone(), capability.clone());
            }
        }

        let mut alias_map = BTreeMap::new();
        for (alias, target) in &aliases.aliases {
            if !valid_alias(alias) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.alias.invalid-format",
                    format!("aliases.{}", bounded_value(alias)),
                    "alias names must be 1 to 128 lowercase ASCII letters, digits, or hyphens, \
                     and must not start with a hyphen",
                ));
            }
            if capability_map.keys().any(|id| id.as_str() == alias) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.alias.collides-with-id",
                    format!("aliases.{}", bounded_value(alias)),
                    "an alias cannot reuse a canonical capability id",
                ));
            }
            if !capability_map.contains_key(target) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.alias.target-unmapped",
                    format!("aliases.{}", bounded_value(alias)),
                    format!(
                        "alias target `{}` does not name a declared capability",
                        bounded_value(target.as_str())
                    ),
                ));
            } else {
                alias_map.insert(alias.clone(), target.clone());
            }
        }

        // NOTE (Gate S1, DIV-6): a capability's `status = "deprecated"` and its
        // presence in `deprecations-v1.toml` are two statements of the same fact
        // in two files, and nothing links them. Enforcement in
        // `validate::validate_capabilities` reads `status` only, so the
        // deprecation list and its `policy` (alias lifetime, removal
        // requirements) are loaded but never consulted. No governed document
        // states which is authoritative, so this is recorded for human
        // ratification rather than enforced here.
        for (index, id) in deprecations.deprecations.iter().enumerate() {
            if !capability_map.contains_key(id) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Taxonomy,
                    "capability.deprecation.unmapped",
                    format!("deprecations[{index}]"),
                    "deprecation entry does not name a declared capability",
                ));
            }
        }

        if !diagnostics.is_empty() {
            sort_diagnostics(&mut diagnostics);
            return Err(diagnostics);
        }

        Ok(Self {
            schema_version: capabilities.schema_version,
            capabilities: capability_map,
            aliases: alias_map,
            deprecations: deprecations.deprecations,
            policy: deprecations.policy,
        })
    }

    /// Returns the taxonomy schema version.
    pub const fn schema_version(&self) -> Version {
        self.schema_version
    }

    /// Returns all canonical capability definitions in deterministic order.
    pub fn capabilities(&self) -> &BTreeMap<CapabilityId, CapabilityDefinition> {
        &self.capabilities
    }

    /// Returns aliases in deterministic order.
    pub fn aliases(&self) -> &BTreeMap<String, CapabilityId> {
        &self.aliases
    }

    /// Returns a canonical capability without allocating for the lookup.
    pub fn capability(&self, id: &CapabilityId) -> Option<&CapabilityDefinition> {
        self.capabilities.get(id)
    }

    /// Returns the canonical target for an alias without allocating for the lookup.
    pub fn alias_target(&self, alias: &str) -> Option<&CapabilityId> {
        self.aliases.get(alias)
    }

    /// Returns the loaded deprecation declarations.
    pub fn deprecations(&self) -> &[CapabilityId] {
        &self.deprecations
    }

    /// Returns the loaded deprecation policy.
    pub const fn policy(&self) -> &DeprecationPolicy {
        &self.policy
    }
}

/// An alias is a single-segment synonym, matched byte-for-byte against manifest
/// capability strings.
///
/// It follows the lexical rule the rest of the taxonomy uses so that two aliases
/// cannot be visually indistinguishable while resolving to different
/// capabilities. The previous check only bounded the length, despite reporting
/// `capability.alias.invalid-format`.
fn valid_alias(alias: &str) -> bool {
    let bytes = alias.as_bytes();
    (1..=128).contains(&bytes.len())
        && bytes
            .first()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}

fn decode_document<T: DeserializeOwned>(
    value: Value,
    document_name: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<T> {
    match value.try_into() {
        Ok(document) => Some(document),
        Err(_) => {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.shape.invalid",
                document_name,
                "taxonomy TOML does not match its governed document shape",
            ));
            None
        }
    }
}

fn read_taxonomy_source(path: &Path, budget: &Budget) -> Result<String, Diagnostic> {
    let metadata = std::fs::metadata(path).map_err(|_| {
        Diagnostic::error(
            DiagnosticClass::Taxonomy,
            "taxonomy.file.stat-failed",
            "$",
            "unable to inspect a taxonomy file",
        )
    })?;
    if metadata.len() > budget.manifest_bytes as u64 {
        return Err(Diagnostic::error(
            DiagnosticClass::Taxonomy,
            "budget.manifest-bytes.exceeded",
            "$",
            "taxonomy file exceeds the configured byte budget",
        ));
    }
    let file = File::open(path).map_err(|_| {
        Diagnostic::error(
            DiagnosticClass::Taxonomy,
            "taxonomy.file.open-failed",
            "$",
            "unable to open a taxonomy file",
        )
    })?;
    let mut bytes = Vec::new();
    file.take(budget.manifest_bytes.saturating_add(1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "taxonomy.file.read-failed",
                "$",
                "unable to read a taxonomy file",
            )
        })?;
    if bytes.len() > budget.manifest_bytes {
        return Err(Diagnostic::error(
            DiagnosticClass::Taxonomy,
            "budget.manifest-bytes.exceeded",
            "$",
            "taxonomy read exceeded the configured byte budget",
        ));
    }
    decode_utf8(bytes, DiagnosticClass::Taxonomy)
}
