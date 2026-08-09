use std::{path::PathBuf, process::id};

use ossus_core::CommitHash;
use ossus_registry::{
    Budget, Diagnostic, DiagnosticSeverity, ManifestValidation, Taxonomy,
    parse_canonical_manifest_with_taxonomy,
};

const EXAMPLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/examples/canonical-manifest.example.toml"
));
const CAPABILITIES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/capabilities-v1.toml"
));
const ALIASES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/aliases-v1.toml"
));
const DEPRECATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/taxonomy/deprecations-v1.toml"
));

fn taxonomy() -> Taxonomy {
    match Taxonomy::load_builtin(&Budget::default()) {
        Ok(taxonomy) => taxonomy,
        Err(diagnostics) => panic!("built-in taxonomy failed to load: {diagnostics:?}"),
    }
}

fn parse(source: &str) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let taxonomy = taxonomy();
    parse_canonical_manifest_with_taxonomy(source, &Budget::default(), &taxonomy)
}

fn codes(diagnostics: &[Diagnostic]) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.reason_code.clone())
        .collect()
}

fn assert_code(source: &str, code: &str) {
    let result = parse(source);
    match result {
        Ok(report) => panic!("expected {code}, got valid report: {report:?}"),
        Err(diagnostics) => assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.reason_code == code),
            "expected {code}, got {:?}",
            codes(&diagnostics)
        ),
    }
}

fn assert_valid(source: &str) {
    let result = parse(source);
    match result {
        Ok(report) => assert!(
            report
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.severity == DiagnosticSeverity::Warning),
            "unexpected fatal diagnostics: {:?}",
            report.diagnostics
        ),
        Err(diagnostics) => panic!("expected valid manifest, got {diagnostics:?}"),
    }
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture did not contain {from:?}");
    source.replacen(from, to, 1)
}

#[test]
fn shipped_example_loads_cleanly() {
    assert_valid(EXAMPLE);
}

#[test]
fn budget_defaults_and_restriction_are_monotonic() {
    let budget = Budget::default();
    assert_eq!(budget.manifest_bytes, 262_144);
    assert_eq!(budget.task_bytes, 65_536);
    assert_eq!(budget.config_bytes, 262_144);
    assert_eq!(budget.max_string_length, 32_768);
    assert_eq!(budget.max_list_items, 2_000);
    assert_eq!(budget.max_nesting_depth, 32);
    assert_eq!(budget.max_manifests_per_source, 50_000);
    assert_eq!(budget.max_project_files, 100_000);
    assert_eq!(budget.max_project_bytes, 67_108_864);

    let other = Budget {
        manifest_bytes: 1,
        task_bytes: 2,
        config_bytes: 3,
        max_string_length: 4,
        max_list_items: 5,
        max_nesting_depth: 6,
        max_manifests_per_source: 7,
        max_project_files: 8,
        max_project_bytes: 9,
    };
    assert_eq!(budget.restrict(&other), other);
    assert_eq!(other.restrict(&budget), other);
}

#[test]
fn taxonomy_loads_all_44_capabilities() {
    let taxonomy = taxonomy();
    assert_eq!(taxonomy.capabilities().len(), 44);
}

#[test]
fn unknown_fields_are_rejected_at_top_and_nested_levels() {
    assert_code(
        &format!("origin = \"external\"\n{EXAMPLE}"),
        "manifest.field.unknown",
    );
    let nested = replace_once(
        EXAMPLE,
        "license = \"MIT\"\n",
        "license = \"MIT\"\nunknown_nested = \"external\"\n",
    );
    assert_code(&nested, "manifest.field.unknown");
}

#[test]
fn origin_fields_are_rejected_at_top_and_under_source() {
    for field in [
        "origin",
        "author_capabilities",
        "upstream_triggers",
        "stars",
    ] {
        let top_level = format!("{field} = \"external\"\n{EXAMPLE}");
        assert_code(&top_level, "manifest.field.unknown");

        let nested = replace_once(
            EXAMPLE,
            "license = \"MIT\"\n",
            &format!("license = \"MIT\"\n{field} = \"external\"\n"),
        );
        assert_code(&nested, "manifest.field.unknown");
    }
}

#[test]
fn unknown_schema_major_fails_closed() {
    let source = replace_once(
        EXAMPLE,
        "schema_version = \"1.0.0\"",
        "schema_version = \"2.0.0\"",
    );
    assert_code(&source, "version.unsupported-major");
}

#[test]
fn missing_required_fields_have_stable_paths() {
    let source = replace_once(EXAMPLE, "schema_version = \"1.0.0\"\n", "");
    let result = parse(&source);
    match result {
        Ok(report) => panic!("missing required field unexpectedly validated: {report:?}"),
        Err(diagnostics) => assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.reason_code == "manifest.field.missing"
                && diagnostic.field_path == "schema_version"
        })),
    }
}

#[test]
fn git_subpaths_require_canonical_relative_posix_form() {
    for subpath in [
        "",
        "/skills/example",
        "skills//example",
        "skills/./example",
        "../example",
        "skills\\example",
    ] {
        let source = replace_once(
            EXAMPLE,
            "subpath = \"skills/frontend-review\"",
            &format!("subpath = {subpath:?}"),
        );
        assert_code(&source, "source.subpath.not-canonical");
    }

    let nul = replace_once(
        EXAMPLE,
        "subpath = \"skills/frontend-review\"",
        "subpath = \"skills\\u0000example\"",
    );
    assert_code(&nul, "source.subpath.not-canonical");
}

#[test]
fn duplicate_and_overlapping_capabilities_are_rejected() {
    let duplicate = replace_once(
        EXAMPLE,
        "required = [\"frontend.visual-design\", \"frontend.responsive-layout\"]",
        "required = [\"frontend.visual-design\", \"frontend.visual-design\"]",
    );
    assert_code(&duplicate, "list.duplicate");

    let overlap = replace_once(
        EXAMPLE,
        "optional = [\"frontend.accessibility\"]",
        "optional = [\"frontend.visual-design\"]",
    );
    assert_code(&overlap, "capability.id.overlap");
}

#[test]
fn excessive_triggers_are_rejected() {
    let triggers = (0..13)
        .map(|index| format!("\"t{index:02}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let source = replace_once(
        EXAMPLE,
        "triggers = [\"review frontend\", \"responsive layout\", \"visual quality\"]",
        &format!("triggers = [{triggers}]"),
    );
    assert_code(&source, "list.count.out-of-range");
}

#[test]
fn structural_budgets_reject_large_strings_lists_and_depth() {
    let long_description = format!("description = \"{}\"", "x".repeat(32_769));
    let source = replace_once(
        EXAMPLE,
        "description = \"Reviews visual quality, responsive layout, and accessibility boundaries.\"",
        &long_description,
    );
    assert_code(&source, "budget.string-length.exceeded");

    let items = (0..2_001).map(|_| "\"xx\"").collect::<Vec<_>>().join(", ");
    let source = replace_once(
        EXAMPLE,
        "triggers = [\"review frontend\", \"responsive layout\", \"visual quality\"]",
        &format!("triggers = [{items}]"),
    );
    assert_code(&source, "budget.list-items.exceeded");

    let mut nested = String::from("a = ");
    for _ in 0..33 {
        nested.push_str("{ a = ");
    }
    nested.push_str("\"x\"");
    for _ in 0..33 {
        nested.push('}');
    }
    assert_code(&nested, "budget.nesting-depth.exceeded");
}

#[test]
fn deep_nesting_is_rejected_before_parser_recursion() {
    let mut tables = String::from("a = ");
    for _ in 0..10_000 {
        tables.push_str("{ a = ");
    }
    tables.push_str("\"x\"");
    for _ in 0..10_000 {
        tables.push('}');
    }
    assert_code(&tables, "budget.nesting-depth.exceeded");

    let mut arrays = String::from("a = ");
    arrays.push_str(&"[".repeat(10_000));
    arrays.push('0');
    arrays.push_str(&"]".repeat(10_000));
    assert_code(&arrays, "budget.nesting-depth.exceeded");
}

#[test]
fn file_size_and_utf8_gates_happen_before_content() {
    let path = temporary_path("oversized");
    let bytes = vec![b'x'; Budget::default().manifest_bytes + 1];
    let write_result = std::fs::write(&path, bytes);
    assert!(write_result.is_ok(), "could not create oversized fixture");
    let result = ossus_registry::load_canonical_manifest(&path, &Budget::default());
    match result {
        Ok(_) => panic!("oversized fixture unexpectedly loaded"),
        Err(diagnostics) => assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.reason_code == "budget.manifest-bytes.exceeded")
        ),
    }
    let _ = std::fs::remove_file(&path);

    let path = temporary_path("utf8");
    let write_result = std::fs::write(&path, [0xff, 0xfe, 0xfd]);
    assert!(write_result.is_ok(), "could not create utf8 fixture");
    let result = ossus_registry::load_canonical_manifest(&path, &Budget::default());
    match result {
        Ok(_) => panic!("invalid UTF-8 fixture unexpectedly loaded"),
        Err(diagnostics) => assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.reason_code == "encoding.utf8.invalid")
        ),
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn conditional_license_rule_distinguishes_shared_and_private_sources() {
    let remote = replace_once(EXAMPLE, "license = \"MIT\"\n", "");
    assert_code(&remote, "source.license.required-for-shared-source");

    let private = replace_once(
        &remote,
        "mode = \"remote-index\"",
        "mode = \"local-private\"",
    );
    assert_valid(&private);
}

#[test]
fn commit_format_and_lowercase_rules_are_enforced() {
    for length in [41, 50, 63] {
        let commit = "a".repeat(length);
        let source = replace_once(EXAMPLE, "0123456789abcdef0123456789abcdef01234567", &commit);
        assert_code(&source, "source.commit.invalid-format");
    }

    let uppercase = replace_once(
        EXAMPLE,
        "0123456789abcdef0123456789abcdef01234567",
        "0123456789ABCDEF0123456789ABCDEF01234567",
    );
    assert_code(&uppercase, "source.commit.not-normalized");

    let approved_uppercase = replace_once(
        EXAMPLE,
        "abcdef0123456789abcdef0123456789abcdef01",
        "ABCDEF0123456789ABCDEF0123456789ABCDEF01",
    );
    assert_code(&approved_uppercase, "review.approved_commit.not-normalized");

    let lower = match CommitHash::parse(&"ab".repeat(20)) {
        Ok(hash) => hash,
        Err(error) => panic!("lowercase commit failed: {error}"),
    };
    let upper = match CommitHash::parse(&"AB".repeat(20)) {
        Ok(hash) => hash,
        Err(error) => panic!("uppercase commit failed: {error}"),
    };
    assert_eq!(lower, upper);
}

#[test]
fn governed_surfaces_are_case_sensitive_and_closed() {
    for invalid in ["claude-code-CLI", "not-a-surface"] {
        let source = replace_once(
            EXAMPLE,
            "agent-skills-standard\", \"claude-code-cli\", \"codex-cli",
            &format!("agent-skills-standard\", \"{invalid}\", \"codex-cli"),
        );
        assert_code(&source, "compatibility.surfaces.unknown");
    }
}

#[test]
fn all_source_distribution_pairs_use_core_policy() {
    let cases = [
        ("remote-index", "source-only", true),
        ("remote-index", "approved-install-only", true),
        ("remote-index", "vendored-redistributable", false),
        ("vendored", "source-only", false),
        ("vendored", "approved-install-only", true),
        ("vendored", "vendored-redistributable", true),
        ("local-private", "source-only", true),
        ("local-private", "approved-install-only", true),
        ("local-private", "vendored-redistributable", false),
    ];
    for (source_mode, distribution_mode, valid) in cases {
        let mut source = replace_once(
            EXAMPLE,
            "mode = \"remote-index\"",
            &format!("mode = \"{source_mode}\""),
        );
        source = replace_once(
            &source,
            "mode = \"source-only\"",
            &format!("mode = \"{distribution_mode}\""),
        );
        if distribution_mode == "vendored-redistributable" {
            source = replace_once(&source, "notice_required = false", "notice_required = true");
        }
        if valid {
            assert_valid(&source);
        } else {
            assert_code(&source, "distribution.mode.contradicts-source-mode");
        }
    }
}

#[test]
fn risk_and_review_tiers_are_floor_checked_but_overdeclaration_passes() {
    let r0_shell = replace_once(EXAMPLE, "instruction-only", "shell-required");
    assert_code(&r0_shell, "risk.tier.below-runtime-requirement");

    let r3_shell = replace_once(
        &replace_once(EXAMPLE, "instruction-only", "shell-required"),
        "tier = \"light-human\"",
        "tier = \"security-human\"",
    );
    let r3_shell = replace_once(&r3_shell, "tier = \"R0\"", "tier = \"R3\"");
    assert_valid(&r3_shell);

    let r4_instruction = replace_once(
        &replace_once(
            EXAMPLE,
            "tier = \"light-human\"",
            "tier = \"security-human\"",
        ),
        "tier = \"R0\"",
        "tier = \"R4\"",
    );
    assert_valid(&r4_instruction);

    let insufficient_review = replace_once(EXAMPLE, "tier = \"R0\"", "tier = \"R3\"");
    assert_code(&insufficient_review, "review.tier.insufficient-for-risk");
}

#[test]
fn r5_is_excluded_from_registry() {
    let source = replace_once(
        &replace_once(EXAMPLE, "tier = \"R0\"", "tier = \"R5\""),
        "tier = \"light-human\"",
        "tier = \"security-human\"",
    );
    assert_code(&source, "risk.tier.excluded-from-registry");
}

#[test]
fn taxonomy_rejects_unmapped_and_alias_capabilities() {
    let unmapped = replace_once(EXAMPLE, "frontend.visual-design", "unknown.capability");
    assert_code(&unmapped, "capability.id.unmapped");

    let alias = replace_once(EXAMPLE, "frontend.visual-design", "project-audit");
    let result = parse(&alias);
    match result {
        Ok(report) => panic!("alias unexpectedly validated: {report:?}"),
        Err(diagnostics) => {
            let diagnostic = diagnostics
                .iter()
                .find(|diagnostic| diagnostic.reason_code == "capability.id.alias-not-canonical");
            match diagnostic {
                Some(diagnostic) => assert!(diagnostic.message.contains("project.discovery")),
                None => panic!("alias diagnostic missing: {diagnostics:?}"),
            }
        }
    }
}

#[test]
fn deprecated_optional_capabilities_warn_and_required_capabilities_fail() {
    let capabilities = format!(
        "{CAPABILITIES}\n[[capabilities]]\nid = \"legacy.capability\"\ndomain = \"legacy\"\nname = \"capability\"\ndefinition = \"A synthetic deprecated capability.\"\nstatus = \"deprecated\"\n"
    );
    let synthetic =
        match Taxonomy::from_toml_sources(&capabilities, ALIASES, DEPRECATIONS, &Budget::default())
        {
            Ok(taxonomy) => taxonomy,
            Err(diagnostics) => panic!("synthetic taxonomy failed: {diagnostics:?}"),
        };

    let optional = replace_once(
        EXAMPLE,
        "optional = [\"frontend.accessibility\"]",
        "optional = [\"legacy.capability\"]",
    );
    let report = parse_canonical_manifest_with_taxonomy(&optional, &Budget::default(), &synthetic);
    match report {
        Ok(report) => assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.reason_code == "capability.id.deprecated"
                && diagnostic.severity == DiagnosticSeverity::Warning
        })),
        Err(diagnostics) => panic!("deprecated optional should warn only: {diagnostics:?}"),
    }

    let required = replace_once(EXAMPLE, "frontend.visual-design", "legacy.capability");
    let result = parse_canonical_manifest_with_taxonomy(&required, &Budget::default(), &synthetic);
    assert_code_with_taxonomy(result, "capability.id.deprecated");
}

fn assert_code_with_taxonomy(result: Result<ManifestValidation, Vec<Diagnostic>>, code: &str) {
    match result {
        Ok(report) => panic!("expected {code}, got valid report: {report:?}"),
        Err(diagnostics) => assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.reason_code == code),
            "expected {code}, got {:?}",
            codes(&diagnostics)
        ),
    }
}

#[test]
fn validation_is_deterministic() {
    let invalid = replace_once(
        &replace_once(EXAMPLE, "instruction-only", "shell-required"),
        "tier = \"R0\"",
        "tier = \"R0\"",
    );
    let first = match parse(&invalid) {
        Ok(report) => report.diagnostics,
        Err(diagnostics) => diagnostics,
    };
    let second = match parse(&invalid) {
        Ok(report) => report.diagnostics,
        Err(diagnostics) => diagnostics,
    };
    assert_eq!(first, second);
}

fn temporary_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ossus-wave-t4-{label}-{}", id()))
}
