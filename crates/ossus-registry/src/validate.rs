//! Semantic validation for typed canonical manifests.

use ossus_core::{
    CapabilityStatus, DistributionMode, ReviewTier, RiskTier, RuntimeRequirement, SourceMode,
    distribution_is_permitted, validate_schema_version,
};

use crate::{
    diagnostic::{
        Diagnostic, DiagnosticClass, DiagnosticSeverity, bounded_value, sort_diagnostics,
    },
    manifest::CanonicalManifest,
    taxonomy::Taxonomy,
};

/// Validates all cross-field and bounded semantic constraints in deterministic order.
pub fn validate_manifest(manifest: &CanonicalManifest, taxonomy: &Taxonomy) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Err(error) = validate_schema_version(&manifest.schema_version) {
        diagnostics.push(Diagnostic::from_contract_error(
            DiagnosticClass::Schema,
            "schema_version",
            &error,
        ));
    }
    if manifest.capability_schema.major() != taxonomy.schema_version().major() {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "capability-schema.version-mismatch",
            "capability_schema",
            "manifest capability schema major does not match the loaded taxonomy",
        ));
    }

    check_optional_string(&manifest.name, "name", 1, 128, &mut diagnostics);
    check_optional_string(
        &manifest.description,
        "description",
        1,
        512,
        &mut diagnostics,
    );
    check_string(
        &manifest.source.repository,
        "source.repository",
        1,
        512,
        &mut diagnostics,
    );
    check_optional_string(
        &manifest.source.subpath,
        "source.subpath",
        1,
        512,
        &mut diagnostics,
    );
    if let Some(subpath) = &manifest.source.subpath
        && !canonical_git_subpath(subpath)
    {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "source.subpath.not-canonical",
            "source.subpath",
            "Git subpath must be relative POSIX form without NUL, empty, '.' or '..' components",
        ));
    }
    check_optional_string(
        &manifest.source.license,
        "source.license",
        0,
        128,
        &mut diagnostics,
    );
    check_optional_string(
        &manifest.risk.notes,
        "risk.notes",
        0,
        1024,
        &mut diagnostics,
    );

    check_count(&manifest.categories, 1, 8, "categories", &mut diagnostics);
    check_duplicates(&manifest.categories, "categories", &mut diagnostics);

    if let Some(triggers) = &manifest.triggers {
        check_count(triggers, 0, 12, "triggers", &mut diagnostics);
        check_duplicates(triggers, "triggers", &mut diagnostics);
        for (index, trigger) in triggers.iter().enumerate() {
            check_string(
                trigger,
                &format!("triggers[{index}]"),
                2,
                80,
                &mut diagnostics,
            );
        }
    }
    if let Some(exclusions) = &manifest.exclusions {
        check_count(exclusions, 0, 12, "exclusions", &mut diagnostics);
        check_duplicates(exclusions, "exclusions", &mut diagnostics);
        for (index, exclusion) in exclusions.iter().enumerate() {
            check_string(
                exclusion,
                &format!("exclusions[{index}]"),
                2,
                120,
                &mut diagnostics,
            );
        }
    }

    check_count(
        &manifest.compatibility.surfaces,
        1,
        16,
        "compatibility.surfaces",
        &mut diagnostics,
    );
    check_duplicates(
        &manifest.compatibility.surfaces,
        "compatibility.surfaces",
        &mut diagnostics,
    );
    check_count(
        &manifest.compatibility.scopes,
        1,
        8,
        "compatibility.scopes",
        &mut diagnostics,
    );
    check_duplicates(
        &manifest.compatibility.scopes,
        "compatibility.scopes",
        &mut diagnostics,
    );

    check_count(
        &manifest.runtime.requirements,
        1,
        12,
        "runtime.requirements",
        &mut diagnostics,
    );
    check_duplicates(
        &manifest.runtime.requirements,
        "runtime.requirements",
        &mut diagnostics,
    );
    if let Some(external_tools) = &manifest.runtime.external_tools {
        check_count(
            external_tools,
            0,
            32,
            "runtime.external_tools",
            &mut diagnostics,
        );
        check_duplicates(external_tools, "runtime.external_tools", &mut diagnostics);
        for (index, tool) in external_tools.iter().enumerate() {
            check_string(
                tool,
                &format!("runtime.external_tools[{index}]"),
                0,
                128,
                &mut diagnostics,
            );
        }
    }

    check_count(
        &manifest.review.reviewer_ids,
        1,
        8,
        "review.reviewer_ids",
        &mut diagnostics,
    );
    check_duplicates(
        &manifest.review.reviewer_ids,
        "review.reviewer_ids",
        &mut diagnostics,
    );
    for (index, reviewer_id) in manifest.review.reviewer_ids.iter().enumerate() {
        check_string(
            reviewer_id,
            &format!("review.reviewer_ids[{index}]"),
            0,
            128,
            &mut diagnostics,
        );
    }

    if let Some(context) = &manifest.context {
        if let Some(measured_tokens) = context.measured_tokens {
            if measured_tokens > 1_000_000 {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Schema,
                    "context.measured_tokens.out-of-range",
                    "context.measured_tokens",
                    "measured token count exceeds the maximum of 1000000",
                ));
            }
        }
        check_optional_string(
            &context.measurement_method,
            "context.measurement_method",
            0,
            128,
            &mut diagnostics,
        );
    }

    if manifest.source.mode != SourceMode::LocalPrivate && manifest.source.license.is_none() {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "source.license.required-for-shared-source",
            "source.license",
            "a shared source must record a license; only local-private may omit it",
        ));
    }

    if !distribution_is_permitted(manifest.source.mode, manifest.distribution.mode) {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "distribution.mode.contradicts-source-mode",
            "distribution.mode",
            "distribution terms contradict the source provenance mode",
        ));
    }
    match manifest.distribution.mode {
        DistributionMode::VendoredRedistributable => {
            if manifest.distribution.notice_required != Some(true) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Schema,
                    "distribution.notice-required.missing-for-redistribution",
                    "distribution.notice_required",
                    "vendored redistribution requires notice_required = true",
                ));
            }
        }
        DistributionMode::SourceOnly | DistributionMode::ApprovedInstallOnly => {
            if manifest.distribution.notice_required == Some(true) {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Schema,
                    "distribution.notice-required.not-applicable",
                    "distribution.notice_required",
                    "notice_required is only applicable to vendored redistribution",
                ));
            }
        }
    }

    if let Some(raw_commit) = &manifest.raw_source_commit {
        if raw_commit != manifest.source.commit.as_str() {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "source.commit.not-normalized",
                "source.commit",
                "source commit must be lowercase in the canonical file",
            ));
        }
    }
    if let Some(raw_commit) = &manifest.raw_approved_commit {
        if raw_commit != manifest.review.approved_commit.as_str() {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "review.approved_commit.not-normalized",
                "review.approved_commit",
                "approved commit must be lowercase in the canonical file",
            ));
        }
    }

    if manifest.risk.tier == RiskTier::R5 {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "risk.tier.excluded-from-registry",
            "risk.tier",
            "R5 is excluded from the stable Registry channel",
        ));
    }

    let required_runtime_tier = manifest
        .runtime
        .requirements
        .iter()
        .map(runtime_minimum_risk)
        .max();
    if let Some(required_runtime_tier) = required_runtime_tier {
        if manifest.risk.tier < required_runtime_tier {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "risk.tier.below-runtime-requirement",
                "risk.tier",
                "declared risk tier is below the minimum implied by runtime requirements",
            ));
        }
    }

    let minimum_review_tier = minimum_review_tier(manifest.risk.tier);
    if manifest.review.tier < minimum_review_tier {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "review.tier.insufficient-for-risk",
            "review.tier",
            "declared review tier is below the minimum required for the risk tier",
        ));
    }

    validate_capabilities(
        &manifest.capabilities.required,
        "capabilities.required",
        true,
        taxonomy,
        &mut diagnostics,
    );
    if let Some(optional) = &manifest.capabilities.optional {
        validate_capabilities(
            optional,
            "capabilities.optional",
            false,
            taxonomy,
            &mut diagnostics,
        );
        for (optional_index, optional_id) in optional.iter().enumerate() {
            if manifest
                .capabilities
                .required
                .iter()
                .any(|required_id| required_id == optional_id)
            {
                diagnostics.push(Diagnostic::error(
                    DiagnosticClass::Schema,
                    "capability.id.overlap",
                    format!("capabilities.optional[{optional_index}]"),
                    "a capability cannot be both required and optional",
                ));
            }
        }
    }
    check_count(
        &manifest.capabilities.required,
        0,
        16,
        "capabilities.required",
        &mut diagnostics,
    );
    check_duplicates(
        &manifest.capabilities.required,
        "capabilities.required",
        &mut diagnostics,
    );
    if let Some(optional) = &manifest.capabilities.optional {
        check_count(optional, 0, 16, "capabilities.optional", &mut diagnostics);
        check_duplicates(optional, "capabilities.optional", &mut diagnostics);
    }

    sort_diagnostics(&mut diagnostics);
    diagnostics
}

fn canonical_git_subpath(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && !value.contains('\0')
        && value
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."))
}

fn validate_capabilities(
    capabilities: &[ossus_core::CapabilityId],
    field_path: &str,
    required: bool,
    taxonomy: &Taxonomy,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (index, id) in capabilities.iter().enumerate() {
        let path = format!("{field_path}[{index}]");
        if let Some(capability) = taxonomy.capability(id) {
            if capability.status == CapabilityStatus::Deprecated {
                if required {
                    diagnostics.push(Diagnostic::error(
                        DiagnosticClass::Taxonomy,
                        "capability.id.deprecated",
                        path,
                        "deprecated capabilities cannot be required",
                    ));
                } else {
                    diagnostics.push(Diagnostic::warning(
                        DiagnosticClass::Taxonomy,
                        "capability.id.deprecated",
                        path,
                        "optional capability is deprecated",
                    ));
                }
            }
        } else if let Some(canonical) = taxonomy.alias_target(id.as_str()) {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "capability.id.alias-not-canonical",
                path,
                format!(
                    "`{}` is an alias, not a canonical capability; use `{}`",
                    bounded_value(id.as_str()),
                    bounded_value(canonical.as_str())
                ),
            ));
        } else {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Taxonomy,
                "capability.id.unmapped",
                path,
                "capability id is not declared in the governed taxonomy",
            ));
        }
    }
}

fn runtime_minimum_risk(requirement: &RuntimeRequirement) -> RiskTier {
    match requirement {
        RuntimeRequirement::InstructionOnly => RiskTier::R0,
        RuntimeRequirement::FilesystemOnly => RiskTier::R1,
        RuntimeRequirement::ShellRequired
        | RuntimeRequirement::NetworkRequired
        | RuntimeRequirement::McpRequired
        | RuntimeRequirement::ExternalCliRequired
        | RuntimeRequirement::HostApiRequired => RiskTier::R3,
    }
}

// Derived from the "Review:" lines of `docs/security/RISK_TIERS.md`. R0 requires
// focused independent agent review; R1 and R2 require complete independent
// agent review, so the schema-v1 compatibility label for R1 floors at
// `full-human`, not `light-human`. ADR-020 defines these strings as legacy wire
// labels and gives the distinct Closure Agent final authority.
const fn minimum_review_tier(risk: RiskTier) -> ReviewTier {
    match risk {
        RiskTier::R0 => ReviewTier::LightHuman,
        RiskTier::R1 | RiskTier::R2 => ReviewTier::FullHuman,
        RiskTier::R3 | RiskTier::R4 | RiskTier::R5 => ReviewTier::SecurityHuman,
    }
}

fn check_count<T>(
    values: &[T],
    minimum: usize,
    maximum: usize,
    field_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if values.len() < minimum || values.len() > maximum {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "list.count.out-of-range",
            field_path,
            format!("list length must be between {minimum} and {maximum} items"),
        ));
    }
}

fn check_duplicates<T: PartialEq>(
    values: &[T],
    field_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for index in 0..values.len() {
        if values[..index]
            .iter()
            .any(|previous| previous == &values[index])
        {
            diagnostics.push(Diagnostic::error(
                DiagnosticClass::Schema,
                "list.duplicate",
                format!("{field_path}[{index}]"),
                "list entries must be unique",
            ));
        }
    }
}

fn check_optional_string(
    value: &Option<String>,
    field_path: &str,
    minimum: usize,
    maximum: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(value) = value {
        check_string(value, field_path, minimum, maximum, diagnostics);
    }
}

fn check_string(
    value: &str,
    field_path: &str,
    minimum: usize,
    maximum: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let length = value.chars().count();
    if length < minimum {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "field.string.too-short",
            field_path,
            format!("string length must be at least {minimum} characters"),
        ));
    }
    if length > maximum {
        diagnostics.push(Diagnostic::error(
            DiagnosticClass::Schema,
            "field.string.too-long",
            field_path,
            format!("string length must be at most {maximum} characters"),
        ));
    }
}

/// Returns whether a diagnostic vector contains a fatal diagnostic.
pub(crate) fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
}
