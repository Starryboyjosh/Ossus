//! Executable evidence for the WAVE-002 canonical-manifest fixture corpus.

use std::{collections::BTreeSet, path::PathBuf, process::id};

use ossus_registry::{
    Budget, Diagnostic, DiagnosticSeverity, ManifestValidation, load_canonical_manifest,
    parse_canonical_manifest_report,
};
use toml::Value;

const INDEX: &str = include_str!("fixtures/INDEX.toml");

#[derive(Debug)]
struct Fixture {
    file: String,
    expectation: String,
    reason_code: Option<String>,
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn indexed_fixtures() -> Vec<Fixture> {
    let document = match toml::from_str::<Value>(INDEX) {
        Ok(document) => document,
        Err(error) => panic!("fixture index is not valid TOML: {error}"),
    };
    let Some(entries) = document.get("fixtures").and_then(Value::as_array) else {
        panic!("fixture index must contain a fixtures array");
    };

    entries
        .iter()
        .map(|entry| {
            let Some(table) = entry.as_table() else {
                panic!("fixture index entry is not a table: {entry:?}");
            };
            Fixture {
                file: required_string(table, "file"),
                expectation: required_string(table, "expectation"),
                reason_code: table
                    .get("reason_code")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
            }
        })
        .collect()
}

fn required_string(table: &toml::map::Map<String, Value>, field: &str) -> String {
    let Some(value) = table.get(field).and_then(Value::as_str) else {
        panic!("fixture index entry is missing string field {field:?}");
    };
    value.to_owned()
}

fn source_for(fixture: &Fixture) -> String {
    let path = fixture_root().join(&fixture.file);
    let mut source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => panic!("could not read {}: {error}", path.display()),
    };

    match fixture.file.as_str() {
        "invalid/budget-long-string.toml" => {
            replace_line(
                &mut source,
                "description = ",
                &format!("description = \"{}\"", "x".repeat(32_769)),
            );
        }
        "invalid/budget-many-items.toml" => {
            let items = (0..2_001)
                .map(|index| format!("\"item-{index:04}\""))
                .collect::<Vec<_>>()
                .join(", ");
            replace_line(
                &mut source,
                "budget_probe = ",
                &format!("budget_probe = [{items}]"),
            );
        }
        _ => {}
    }
    source
}

fn replace_line(source: &mut String, prefix: &str, replacement: &str) {
    let Some(original) = source.lines().find(|line| line.starts_with(prefix)) else {
        panic!("amplification seed did not contain line prefix {prefix:?}");
    };
    *source = source.replacen(original, replacement, 1);
}

fn diagnostics(result: Result<ManifestValidation, Vec<Diagnostic>>) -> Vec<Diagnostic> {
    match result {
        Ok(report) => report.diagnostics,
        Err(diagnostics) => diagnostics,
    }
}

#[test]
fn every_indexed_fixture_has_its_exact_expected_outcome() {
    for fixture in indexed_fixtures() {
        let source = source_for(&fixture);
        let result = parse_canonical_manifest_report(&source, &Budget::default());
        match fixture.expectation.as_str() {
            "valid" => match result {
                Ok(report) => assert!(
                    report
                        .diagnostics
                        .iter()
                        .all(|diagnostic| diagnostic.severity == DiagnosticSeverity::Warning),
                    "{} produced fatal diagnostics: {:?}",
                    fixture.file,
                    report.diagnostics
                ),
                Err(diagnostics) => {
                    panic!("{} should be valid: {diagnostics:?}", fixture.file)
                }
            },
            "invalid" => {
                let Some(expected) = fixture.reason_code.as_deref() else {
                    panic!("{} has no expected reason code", fixture.file);
                };
                let actual = diagnostics(result);
                assert!(
                    actual
                        .iter()
                        .any(|diagnostic| diagnostic.reason_code == expected),
                    "{} expected {expected:?}, got {actual:?}",
                    fixture.file
                );
            }
            other => panic!("{} has unknown expectation {other:?}", fixture.file),
        }
    }
}

#[test]
fn approved_commit_format_error_is_attributed_to_the_review_field() {
    let fixture = Fixture {
        file: "invalid/approved-commit-abbreviated.toml".to_owned(),
        expectation: "invalid".to_owned(),
        reason_code: Some("review.approved_commit.invalid-format".to_owned()),
    };
    let actual = diagnostics(parse_canonical_manifest_report(
        &source_for(&fixture),
        &Budget::default(),
    ));

    assert!(
        actual.iter().any(|diagnostic| {
            diagnostic.reason_code == "review.approved_commit.invalid-format"
                && diagnostic.field_path == "review.approved_commit"
        }),
        "approved commit error was attributed to the wrong field: {actual:?}"
    );
    assert!(
        actual
            .iter()
            .all(|diagnostic| diagnostic.reason_code != "source.commit.invalid-format"),
        "approved commit error reused the source commit reason code: {actual:?}"
    );
}

#[test]
fn fixture_inventory_is_complete_and_has_no_duplicate_paths() {
    let indexed = indexed_fixtures()
        .into_iter()
        .map(|fixture| fixture.file)
        .collect::<BTreeSet<_>>();
    let mut present = BTreeSet::new();
    for directory in ["valid", "invalid"] {
        let path = fixture_root().join(directory);
        let entries = match std::fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(error) => panic!("could not list {}: {error}", path.display()),
        };
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => panic!("could not read fixture directory entry: {error}"),
            };
            let path = entry.path();
            if path.extension().and_then(|extension| extension.to_str()) == Some("toml") {
                let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                    panic!("fixture path is not valid UTF-8: {}", path.display());
                };
                assert!(
                    present.insert(format!("{directory}/{name}")),
                    "duplicate fixture path: {}",
                    path.display()
                );
            }
        }
    }
    assert_eq!(indexed, present, "INDEX.toml and the fixture tree diverged");
}

#[test]
fn f12_origin_claims_never_deserialize_into_canonical_state() {
    for file in [
        "invalid/origin-top-level.toml",
        "invalid/author-capabilities-top-level.toml",
        "invalid/upstream-triggers-top-level.toml",
        "invalid/stars-top-level.toml",
    ] {
        let fixture = Fixture {
            file: file.to_owned(),
            expectation: "invalid".to_owned(),
            reason_code: Some("manifest.field.unknown".to_owned()),
        };
        let result = parse_canonical_manifest_report(&source_for(&fixture), &Budget::default());
        let actual = diagnostics(result);
        assert!(
            actual.iter().any(|diagnostic| {
                diagnostic.reason_code == "manifest.field.unknown"
                    && diagnostic.class == ossus_registry::DiagnosticClass::Schema
            }),
            "{file} entered canonical state instead of failing closed: {actual:?}"
        );
    }
}

#[test]
fn generated_invalid_utf8_case_fails_before_toml_parsing() {
    let document = match toml::from_str::<Value>(INDEX) {
        Ok(document) => document,
        Err(error) => panic!("fixture index is not valid TOML: {error}"),
    };
    let Some(case) = document
        .get("generated_cases")
        .and_then(Value::as_array)
        .and_then(|cases| cases.first())
        .and_then(Value::as_table)
    else {
        panic!("fixture index must contain the generated invalid-utf8 case");
    };
    assert_eq!(
        case.get("case").and_then(Value::as_str),
        Some("invalid-utf8")
    );
    let Some(expected) = case.get("reason_code").and_then(Value::as_str) else {
        panic!("generated invalid-utf8 case has no reason code");
    };

    let path = std::env::temp_dir().join(format!("ossus-wave-t5-invalid-utf8-{}", id()));
    let write_result = std::fs::write(&path, [0xff, 0xfe, 0xfd]);
    assert!(
        write_result.is_ok(),
        "could not create invalid UTF-8 fixture"
    );
    let result = load_canonical_manifest(&path, &Budget::default());
    let _ = std::fs::remove_file(&path);
    let Err(actual) = result else {
        panic!("invalid UTF-8 unexpectedly entered canonical state");
    };
    assert!(
        actual
            .iter()
            .any(|diagnostic| diagnostic.reason_code == expected),
        "expected {expected:?}, got {actual:?}"
    );
}
