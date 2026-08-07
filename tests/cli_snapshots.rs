#![allow(
    clippy::expect_used,
    reason = "CLI integration tests use expect for deterministic fixture setup"
)]

use std::fs;
use std::path::Path;

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

fn command_output(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ossus"))
        .args(arguments)
        .output()
        .expect("ossus binary should be runnable")
}

fn cli_output(arguments: &[&str]) -> String {
    let output = command_output(arguments);
    let status = output
        .status
        .code()
        .map_or_else(|| "signal".to_owned(), |code| code.to_string());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    format!("status: {status}\nstdout:\n{stdout}stderr:\n{stderr}")
}

fn fixture_index() -> (TempDir, String) {
    let temporary = TempDir::new().expect("temporary directory should be created");
    let manifest_root = temporary.path().join("manifests");
    fs::create_dir(&manifest_root).expect("manifest directory should be created");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../catalog/examples/canonical-manifest.example.toml"),
        manifest_root.join("example.toml"),
    )
    .expect("example manifest should be copied");
    let index = temporary.path().join("registry.sqlite3");
    let manifest_root = manifest_root.to_string_lossy().into_owned();
    let index_text = index.to_string_lossy().into_owned();
    let output = command_output(&[
        "--format",
        "json",
        "registry",
        "reindex",
        "--manifest-root",
        &manifest_root,
        "--index",
        &index_text,
    ]);
    assert!(output.status.success(), "fixture reindex should succeed");
    (temporary, index_text)
}

fn json_stdout(arguments: &[&str]) -> Value {
    let output = command_output(arguments);
    assert!(output.status.success(), "JSON command should succeed");
    serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON")
}

#[test]
fn version_output() {
    insta::assert_snapshot!(cli_output(&["--version"]));
}

#[test]
fn root_help_output() {
    insta::assert_snapshot!(cli_output(&["--help"]));
}

#[test]
fn registry_help_output() {
    insta::assert_snapshot!(cli_output(&["registry", "--help"]));
}

#[test]
fn search_help_output() {
    insta::assert_snapshot!(cli_output(&["search", "--help"]));
}

#[test]
fn show_help_output() {
    insta::assert_snapshot!(cli_output(&["show", "--help"]));
}

#[test]
fn registry_commands_emit_versioned_json() {
    let (_temporary, index) = fixture_index();
    let status = json_stdout(&["--format", "json", "registry", "status", "--index", &index]);
    assert_eq!(status["schema_version"], "1.0.0");
    assert_eq!(status["compatible"], true);
    assert_eq!(status["resource_count"], 1);
    assert_eq!(status["fts5_available"], true);
}

#[test]
fn search_and_show_use_the_local_index() {
    let (_temporary, index) = fixture_index();
    let search = json_stdout(&[
        "--format",
        "json",
        "search",
        "Frontend",
        "--capability",
        "frontend.visual-design",
        "--category",
        "frontend",
        "--surface",
        "claude-code-cli",
        "--source-mode",
        "remote-index",
        "--runtime",
        "instruction-only",
        "--risk-max",
        "R0",
        "--index",
        &index,
    ]);
    assert_eq!(search["schema_version"], "1.0.0");
    assert_eq!(search["results"].as_array().map(Vec::len), Some(1));
    assert_eq!(search["results"][0]["id"], "ossus-example.frontend-review");

    let show = json_stdout(&[
        "--format",
        "json",
        "show",
        "ossus-example.frontend-review",
        "--index",
        &index,
    ]);
    assert_eq!(show["schema_version"], "1.0.0");
    assert_eq!(show["resource"]["id"], "ossus-example.frontend-review");
}

#[test]
fn registry_cli_exit_codes_are_stable() {
    let temporary = TempDir::new().expect("temporary directory should be created");
    let missing_index = temporary.path().join("missing.sqlite3");
    let missing_index = missing_index.to_string_lossy().into_owned();
    let missing_status = command_output(&["registry", "status", "--index", &missing_index]);
    assert_eq!(missing_status.status.code(), Some(20));

    let invalid_option = command_output(&["search", "--unknown", "value"]);
    assert_eq!(invalid_option.status.code(), Some(2));

    let (_temporary, index) = fixture_index();
    let missing_resource = command_output(&["show", "resource.missing", "--index", &index]);
    assert_eq!(missing_resource.status.code(), Some(20));
}

macro_rules! placeholder_snapshot_tests {
    ($($name:ident => $command:literal),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                insta::assert_snapshot!(cli_output(&[$command, "--help"]));
            }
        )+
    };
}

placeholder_snapshot_tests! {
    placeholder_init => "init",
    placeholder_config => "config",
    placeholder_scan => "scan",
    placeholder_resolve => "resolve",
    placeholder_explain => "explain",
    placeholder_activate => "activate",
    placeholder_deactivate => "deactivate",
    placeholder_lock => "lock",
    placeholder_doctor => "doctor",
    placeholder_eval => "eval",
    placeholder_audit => "audit",
    placeholder_research => "research",
}
