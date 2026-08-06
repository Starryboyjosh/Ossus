#![allow(
    clippy::expect_used,
    reason = "CLI integration tests use expect for deterministic fixture setup"
)]

use std::{path::PathBuf, process::id};

use assert_cmd::Command;

const EXAMPLE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/examples/canonical-manifest.example.toml"
);

fn command(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ossus"))
        .args(arguments)
        .output()
        .expect("ossus binary should be runnable")
}

#[test]
fn validate_accepts_shipped_example() {
    let output = command(&["validate", EXAMPLE_PATH]);
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains(": valid"));
}

#[test]
fn validate_requires_a_path() {
    let output = command(&["validate"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("at least one manifest PATH"));
}

#[test]
fn schema_errors_use_exit_code_11_and_json_is_versioned() {
    let output = command(&[
        "--format",
        "json",
        "validate",
        "/definitely/not/a/manifest.toml",
    ]);
    assert_eq!(output.status.code(), Some(11));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"schema_version\":\"1.0.0\""));
    assert!(stdout.contains("\"reason_code\":\"manifest.file.stat-failed\""));
}

#[test]
fn taxonomy_errors_use_exit_code_12() {
    let path = temporary_path("taxonomy");
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specs/examples/canonical-manifest.example.toml"
    ))
    .replace("frontend.visual-design", "unknown.capability");
    let write_result = std::fs::write(&path, source);
    assert!(write_result.is_ok(), "could not create taxonomy fixture");

    let path_string = path.to_string_lossy().into_owned();
    let output = command(&["validate", &path_string]);
    assert_eq!(output.status.code(), Some(12));
    assert!(String::from_utf8_lossy(&output.stderr).contains("capability.id.unmapped"));
    let _ = std::fs::remove_file(path);
}

fn temporary_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ossus-cli-validate-{label}-{}", id()))
}
