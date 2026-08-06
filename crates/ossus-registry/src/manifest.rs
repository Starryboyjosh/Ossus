//! Typed canonical manifests and bounded loading entry points.

use std::{fs::File, io::Read, path::Path};

use ossus_core::{
    CapabilityId, CategoryName, CommitHash, DistributionMode, Portability, ResourceId,
    ResourceType, ReviewStatus, ReviewTier, RiskTier, RuntimeRequirement, Scope, SourceMode,
    Surface, TreeHash, Version,
};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::{
    budget::{Budget, decode_utf8, parse_bounded_toml},
    diagnostic::{Diagnostic, DiagnosticClass, bounded_value, sort_diagnostics},
    taxonomy::Taxonomy,
    validate::{has_errors, validate_manifest},
};

/// A typed canonical manifest accepted by the trusted Registry boundary.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalManifest {
    pub schema_version: Version,
    pub capability_schema: Version,
    pub id: ResourceId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub source: Source,
    pub capabilities: Capabilities,
    pub categories: Vec<CategoryName>,
    pub triggers: Option<Vec<String>>,
    pub exclusions: Option<Vec<String>>,
    pub compatibility: Compatibility,
    pub runtime: Runtime,
    pub risk: Risk,
    pub review: Review,
    pub context: Option<Context>,
    pub distribution: Distribution,
    #[serde(skip)]
    pub(crate) raw_source_commit: Option<String>,
    #[serde(skip)]
    pub(crate) raw_approved_commit: Option<String>,
}

struct RawAliasUse {
    field_path: String,
    alias: String,
    canonical: String,
}

/// Immutable source provenance and content identity.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub mode: SourceMode,
    pub repository: String,
    pub commit: CommitHash,
    pub subpath: Option<String>,
    pub tree_hash: TreeHash,
    pub license: Option<String>,
}

/// Required and optional governed capabilities.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub required: Vec<CapabilityId>,
    pub optional: Option<Vec<CapabilityId>>,
}

/// Host and installation compatibility dimensions.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Compatibility {
    pub surfaces: Vec<Surface>,
    pub portability: Portability,
    pub scopes: Vec<Scope>,
}

/// Runtime requirements and external command dependencies.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Runtime {
    pub requirements: Vec<RuntimeRequirement>,
    pub external_tools: Option<Vec<String>>,
}

/// Declared risk tier and bounded reviewer notes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Risk {
    pub tier: RiskTier,
    pub notes: Option<String>,
}

/// Review status, required review depth, immutable approval commit, and reviewers.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    pub status: ReviewStatus,
    pub tier: ReviewTier,
    pub approved_commit: CommitHash,
    pub reviewer_ids: Vec<String>,
}

/// Optional context measurement metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub measured_tokens: Option<u64>,
    pub measurement_method: Option<String>,
}

/// Redistribution terms for a source reference.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Distribution {
    pub mode: DistributionMode,
    pub notice_required: Option<bool>,
}

/// A successfully parsed manifest and any non-fatal diagnostics.
#[derive(Clone, Debug)]
pub struct ManifestValidation {
    pub manifest: CanonicalManifest,
    pub diagnostics: Vec<Diagnostic>,
}

/// Loads a canonical manifest from a path using the built-in taxonomy.
pub fn load_canonical_manifest(
    path: &Path,
    budget: &Budget,
) -> Result<CanonicalManifest, Vec<Diagnostic>> {
    Ok(load_canonical_manifest_report(path, budget)?.manifest)
}

/// Loads a canonical manifest and retains non-fatal diagnostics for CLI/reporting.
pub fn load_canonical_manifest_report(
    path: &Path,
    budget: &Budget,
) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let source = read_manifest_source(path, budget)?;
    parse_canonical_manifest_report(&source, budget)
}

/// Parses a canonical manifest source using the built-in taxonomy.
pub fn parse_canonical_manifest(
    source: &str,
    budget: &Budget,
) -> Result<CanonicalManifest, Vec<Diagnostic>> {
    Ok(parse_canonical_manifest_report(source, budget)?.manifest)
}

/// Parses a canonical manifest and retains non-fatal diagnostics.
pub fn parse_canonical_manifest_report(
    source: &str,
    budget: &Budget,
) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let value = parse_bounded_toml(source, budget, DiagnosticClass::Schema)?;
    let taxonomy = Taxonomy::load_builtin(budget)?;
    parse_manifest_value(value, &taxonomy)
}

/// Parses a canonical manifest against an explicitly supplied taxonomy.
pub fn parse_canonical_manifest_with_taxonomy(
    source: &str,
    budget: &Budget,
    taxonomy: &Taxonomy,
) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let value = parse_bounded_toml(source, budget, DiagnosticClass::Schema)?;
    parse_manifest_value(value, taxonomy)
}

fn parse_manifest_value(
    value: Value,
    taxonomy: &Taxonomy,
) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let mut diagnostics = unknown_field_diagnostics(&value);
    diagnostics.extend(required_field_diagnostics(&value));
    if !diagnostics.is_empty() {
        sort_diagnostics(&mut diagnostics);
        return Err(diagnostics);
    }

    let mut normalized_value = value;
    let aliases = normalize_aliases(&mut normalized_value, taxonomy);

    let mut typed_diagnostics = Vec::new();
    validate_raw_core_fields(&normalized_value, &mut typed_diagnostics);
    if !typed_diagnostics.is_empty() {
        sort_diagnostics(&mut typed_diagnostics);
        return Err(typed_diagnostics);
    }

    let mut manifest = match normalized_value.clone().try_into::<CanonicalManifest>() {
        Ok(manifest) => manifest,
        Err(_) => {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Schema,
                "manifest.shape.invalid",
                "$",
                "TOML values do not match the canonical manifest contract",
            )]);
        }
    };

    manifest.raw_source_commit = raw_string(&normalized_value, &["source", "commit"]);
    manifest.raw_approved_commit = raw_string(&normalized_value, &["review", "approved_commit"]);

    diagnostics = validate_manifest(&manifest, taxonomy);
    for alias in aliases {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Taxonomy,
            "capability.id.alias-not-canonical",
            alias.field_path,
            format!(
                "`{}` is an alias, not a canonical capability; use `{}`",
                alias.alias, alias.canonical
            ),
        ));
    }
    sort_diagnostics(&mut diagnostics);
    if has_errors(&diagnostics) {
        Err(diagnostics)
    } else {
        Ok(ManifestValidation {
            manifest,
            diagnostics,
        })
    }
}

fn normalize_aliases(value: &mut Value, taxonomy: &Taxonomy) -> Vec<RawAliasUse> {
    let mut aliases = Vec::new();
    for field in ["required", "optional"] {
        let Some(capability_table) = value
            .as_table_mut()
            .and_then(|table| table.get_mut("capabilities"))
            .and_then(Value::as_table_mut)
        else {
            continue;
        };
        let Some(items) = capability_table
            .get_mut(field)
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        for (index, item) in items.iter_mut().enumerate() {
            let Some(raw) = item.as_str() else {
                continue;
            };
            let Some(canonical) = taxonomy.alias_target(raw) else {
                continue;
            };
            aliases.push(RawAliasUse {
                field_path: format!("capabilities.{field}[{index}]"),
                alias: raw.to_owned(),
                canonical: canonical.as_str().to_owned(),
            });
            *item = Value::String(canonical.as_str().to_owned());
        }
    }
    aliases
}

fn read_manifest_source(path: &Path, budget: &Budget) -> Result<String, Vec<Diagnostic>> {
    let metadata = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Schema,
                "manifest.file.stat-failed",
                "$",
                "unable to inspect the manifest file",
            )]);
        }
    };

    let limit = budget.manifest_bytes as u64;
    if metadata.len() > limit {
        return Err(vec![Diagnostic::error(
            DiagnosticClass::Schema,
            "budget.manifest-bytes.exceeded",
            "$",
            "manifest file exceeds the configured byte budget",
        )]);
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(vec![Diagnostic::error(
                DiagnosticClass::Schema,
                "manifest.file.open-failed",
                "$",
                "unable to open the manifest file",
            )]);
        }
    };

    // A metadata check is only a first gate. The bounded read protects against
    // a file growing between stat and open, or a filesystem reporting stale size.
    let bounded_read = budget.manifest_bytes.saturating_add(1) as u64;
    let mut bytes = Vec::new();
    let read_result = file.by_ref().take(bounded_read).read_to_end(&mut bytes);
    if read_result.is_err() {
        return Err(vec![Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.file.read-failed",
            "$",
            "unable to read the manifest file",
        )]);
    }
    if bytes.len() > budget.manifest_bytes {
        return Err(vec![Diagnostic::error(
            DiagnosticClass::Schema,
            "budget.manifest-bytes.exceeded",
            "$",
            "manifest read exceeded the configured byte budget",
        )]);
    }

    match decode_utf8(bytes, DiagnosticClass::Schema) {
        Ok(source) => Ok(source),
        Err(diagnostic) => Err(vec![diagnostic]),
    }
}

fn raw_string(value: &Value, path: &[&str]) -> Option<String> {
    raw_value(value, path)?.as_str().map(ToOwned::to_owned)
}

fn raw_value<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        let table = current.as_table()?;
        current = table.get(*segment)?;
    }
    Some(current)
}

fn push_contract_error(
    diagnostics: &mut Vec<Diagnostic>,
    field_path: impl Into<String>,
    result: Result<(), ossus_core::ContractError>,
) {
    if let Err(error) = result {
        let field_path = field_path.into();
        if field_path == "review.approved_commit"
            && matches!(error, ossus_core::ContractError::InvalidCommitHash { .. })
        {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "review.approved_commit.invalid-format",
                field_path,
                "approved commit must be a full 40- or 64-character hexadecimal identity",
            ));
        } else {
            diagnostics.push(Diagnostic::from_contract_error(
                DiagnosticClass::Schema,
                field_path,
                &error,
            ));
        }
    }
}

fn validate_raw_core_fields(value: &Value, diagnostics: &mut Vec<Diagnostic>) {
    check_raw_scalar(
        value,
        &["schema_version"],
        "schema_version",
        |raw| Version::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["capability_schema"],
        "capability_schema",
        |raw| Version::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["id"],
        "id",
        |raw| ResourceId::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["resource_type"],
        "resource_type",
        |raw| ResourceType::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["source", "mode"],
        "source.mode",
        |raw| SourceMode::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["source", "commit"],
        "source.commit",
        |raw| CommitHash::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["source", "tree_hash"],
        "source.tree_hash",
        |raw| TreeHash::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["capabilities", "required"],
        "capabilities.required",
        |raw| CapabilityId::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["capabilities", "optional"],
        "capabilities.optional",
        |raw| CapabilityId::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["categories"],
        "categories",
        |raw| CategoryName::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["compatibility", "surfaces"],
        "compatibility.surfaces",
        |raw| Surface::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["compatibility", "portability"],
        "compatibility.portability",
        |raw| Portability::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["compatibility", "scopes"],
        "compatibility.scopes",
        |raw| Scope::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_array(
        value,
        &["runtime", "requirements"],
        "runtime.requirements",
        |raw| RuntimeRequirement::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["risk", "tier"],
        "risk.tier",
        |raw| RiskTier::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["review", "status"],
        "review.status",
        |raw| ReviewStatus::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["review", "tier"],
        "review.tier",
        |raw| ReviewTier::try_from(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["review", "approved_commit"],
        "review.approved_commit",
        |raw| CommitHash::parse(raw).map(|_| ()),
        diagnostics,
    );
    check_raw_scalar(
        value,
        &["distribution", "mode"],
        "distribution.mode",
        |raw| DistributionMode::try_from(raw).map(|_| ()),
        diagnostics,
    );
}

fn check_raw_scalar<F>(
    value: &Value,
    path: &[&str],
    field_path: &str,
    parser: F,
    diagnostics: &mut Vec<Diagnostic>,
) where
    F: FnOnce(&str) -> Result<(), ossus_core::ContractError>,
{
    let Some(raw_value) = raw_value(value, path) else {
        return;
    };
    let Some(raw) = raw_value.as_str() else {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.field.shape",
            field_path,
            "field must be a string",
        ));
        return;
    };
    push_contract_error(diagnostics, field_path, parser(raw));
}

fn check_raw_array<F>(
    value: &Value,
    path: &[&str],
    field_path: &str,
    parser: F,
    diagnostics: &mut Vec<Diagnostic>,
) where
    F: Fn(&str) -> Result<(), ossus_core::ContractError> + Copy,
{
    let Some(raw_value) = raw_value(value, path) else {
        return;
    };
    let Some(array) = raw_value.as_array() else {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.field.shape",
            field_path,
            "field must be an array of strings",
        ));
        return;
    };
    for (index, item) in array.iter().enumerate() {
        let Some(raw) = item.as_str() else {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "manifest.field.shape",
                format!("{field_path}[{index}]"),
                "list entry must be a string",
            ));
            continue;
        };
        push_contract_error(diagnostics, format!("{field_path}[{index}]"), parser(raw));
    }
}

fn required_field_diagnostics(value: &Value) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let Some(root) = value.as_table() else {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.field.shape",
            "$",
            "canonical manifest root must be a TOML table",
        ));
        return diagnostics;
    };

    check_required_fields(
        root,
        "",
        &[
            "schema_version",
            "capability_schema",
            "id",
            "resource_type",
            "source",
            "capabilities",
            "categories",
            "compatibility",
            "runtime",
            "risk",
            "review",
            "distribution",
        ],
        &mut diagnostics,
    );
    check_nested_required(
        root,
        "source",
        &["mode", "repository", "commit", "tree_hash"],
        &mut diagnostics,
    );
    check_nested_required(root, "capabilities", &["required"], &mut diagnostics);
    check_nested_required(
        root,
        "compatibility",
        &["surfaces", "portability", "scopes"],
        &mut diagnostics,
    );
    check_nested_required(root, "runtime", &["requirements"], &mut diagnostics);
    check_nested_required(root, "risk", &["tier"], &mut diagnostics);
    check_nested_required(
        root,
        "review",
        &["status", "tier", "approved_commit", "reviewer_ids"],
        &mut diagnostics,
    );
    check_nested_required(root, "distribution", &["mode"], &mut diagnostics);
    check_optional_object(root, "context", &mut diagnostics);
    diagnostics
}

fn check_nested_required(
    root: &toml::map::Map<String, Value>,
    field: &str,
    required: &[&str],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = root.get(field) else {
        return;
    };
    let Some(table) = value.as_table() else {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.field.shape",
            field,
            "nested manifest section must be a TOML table",
        ));
        return;
    };
    check_required_fields(table, field, required, diagnostics);
}

fn check_optional_object(
    root: &toml::map::Map<String, Value>,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(value) = root.get(field)
        && !value.is_table()
    {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "manifest.field.shape",
            field,
            "nested manifest section must be a TOML table",
        ));
    }
}

fn check_required_fields(
    table: &toml::map::Map<String, Value>,
    base_path: &str,
    fields: &[&str],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for field in fields {
        if !table.contains_key(*field) {
            let field_path = if base_path.is_empty() {
                (*field).to_owned()
            } else {
                format!("{base_path}.{field}")
            };
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "manifest.field.missing",
                field_path,
                "required field is missing",
            ));
        }
    }
}

#[derive(Clone, Copy)]
enum ObjectKind {
    Root,
    Source,
    Capabilities,
    Compatibility,
    Runtime,
    Risk,
    Review,
    Context,
    Distribution,
}

fn unknown_field_diagnostics(value: &Value) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut pending = vec![(value, ObjectKind::Root, String::from("$"))];

    while let Some((current, kind, base_path)) = pending.pop() {
        let Some(table) = current.as_table() else {
            continue;
        };
        for (key, child) in table.iter().rev() {
            match child_kind(kind, key) {
                None => {
                    let field_path = if base_path == "$" {
                        bounded_value(key)
                    } else {
                        format!("{base_path}.{}", bounded_value(key))
                    };
                    diagnostics.push(Diagnostic::error(
                        DiagnosticClass::Schema,
                        "manifest.field.unknown",
                        field_path,
                        format!(
                            "unknown field `{}` is not part of the canonical manifest contract",
                            bounded_value(key)
                        ),
                    ));
                }
                Some(None) => {}
                Some(Some(child_kind)) => {
                    let child_path = if base_path == "$" {
                        bounded_value(key)
                    } else {
                        format!("{base_path}.{}", bounded_value(key))
                    };
                    pending.push((child, child_kind, child_path));
                }
            }
        }
    }

    diagnostics
}

fn child_kind(kind: ObjectKind, key: &str) -> Option<Option<ObjectKind>> {
    let child = match kind {
        ObjectKind::Root => match key {
            "source" => Some(ObjectKind::Source),
            "capabilities" => Some(ObjectKind::Capabilities),
            "compatibility" => Some(ObjectKind::Compatibility),
            "runtime" => Some(ObjectKind::Runtime),
            "risk" => Some(ObjectKind::Risk),
            "review" => Some(ObjectKind::Review),
            "context" => Some(ObjectKind::Context),
            "distribution" => Some(ObjectKind::Distribution),
            "schema_version" | "capability_schema" | "id" | "name" | "description"
            | "resource_type" | "categories" | "triggers" | "exclusions" => None,
            _ => return None,
        },
        ObjectKind::Source => match key {
            "mode" | "repository" | "commit" | "subpath" | "tree_hash" | "license" => None,
            _ => return None,
        },
        ObjectKind::Capabilities => match key {
            "required" | "optional" => None,
            _ => return None,
        },
        ObjectKind::Compatibility => match key {
            "surfaces" | "portability" | "scopes" => None,
            _ => return None,
        },
        ObjectKind::Runtime => match key {
            "requirements" | "external_tools" => None,
            _ => return None,
        },
        ObjectKind::Risk => match key {
            "tier" | "notes" => None,
            _ => return None,
        },
        ObjectKind::Review => match key {
            "status" | "tier" | "approved_commit" | "reviewer_ids" => None,
            _ => return None,
        },
        ObjectKind::Context => match key {
            "measured_tokens" | "measurement_method" => None,
            _ => return None,
        },
        ObjectKind::Distribution => match key {
            "mode" | "notice_required" => None,
            _ => return None,
        },
    };
    Some(child)
}
