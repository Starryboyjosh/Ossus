#![allow(
    clippy::expect_used,
    reason = "CLI integration tests use expect for deterministic fixture setup"
)]

use std::path::Path;
use std::{collections::BTreeSet, fs};

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

fn json_output(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON")
}

fn assert_object_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("output should be a JSON object")
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|key| (*key).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_resource_output_contract(resource: &Value) {
    assert_object_keys(
        resource,
        &[
            "id",
            "name",
            "description",
            "resource_type",
            "source_mode",
            "source_repository",
            "source_commit",
            "source_subpath",
            "tree_hash",
            "capabilities",
            "categories",
            "surfaces",
            "runtimes",
            "risk_tier",
            "review_status",
            "distribution_mode",
        ],
    );
    assert!(resource["id"].is_string());
    assert!(resource["name"].is_string() || resource["name"].is_null());
    assert!(resource["description"].is_string() || resource["description"].is_null());
    assert!(resource["source_commit"].as_str().is_some_and(
        |commit| commit.len() >= 40 && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
    ));
    assert!(
        resource["tree_hash"]
            .as_str()
            .is_some_and(|hash| hash.starts_with("sha256:") && hash.len() == 71)
    );
    for field in ["capabilities", "categories", "surfaces", "runtimes"] {
        assert!(
            resource[field]
                .as_array()
                .is_some_and(|values| values.iter().all(Value::is_string))
        );
    }
}

fn reindex_json(manifest_root: &Path, index: &Path) -> Value {
    let manifest_root = manifest_root.to_string_lossy().into_owned();
    let index = index.to_string_lossy().into_owned();
    json_stdout(&[
        "--format",
        "json",
        "registry",
        "reindex",
        "--manifest-root",
        &manifest_root,
        "--index",
        &index,
    ])
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

#[test]
fn registry_cli_human_outputs_cover_all_wave003_commands() {
    let (temporary, index) = fixture_index();
    let manifest_root = temporary.path().join("manifests");

    let status = command_output(&["registry", "status", "--index", &index]);
    assert!(status.status.success());
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(status_stdout.contains("status: local index is healthy\n"));
    assert!(status_stdout.contains("schema_version: 1\n"));
    assert!(status_stdout.contains("resources: 1\n"));
    assert!(status_stdout.contains("excluded: 0\n"));
    assert!(status_stdout.contains("fts5: available\n"));
    assert!(status_stdout.contains("reindex_required: false\n"));

    let search = command_output(&[
        "search",
        "--exact",
        "ossus-example.frontend-review",
        "--index",
        &index,
    ]);
    assert!(search.status.success());
    assert_eq!(
        String::from_utf8_lossy(&search.stdout),
        "ossus-example.frontend-review\tR0\tFrontend Review\n"
    );

    let show = command_output(&["show", "ossus-example.frontend-review", "--index", &index]);
    assert!(show.status.success());
    assert_eq!(
        String::from_utf8_lossy(&show.stdout),
        concat!(
            "id: ossus-example.frontend-review\n",
            "name: Frontend Review\n",
            "type: skill\n",
            "description: Reviews visual quality, responsive layout, and accessibility boundaries.\n",
            "source: https://example.invalid/frontend-review.git@0123456789abcdef0123456789abcdef01234567\n",
            "risk: R0\n",
            "review: approved\n",
            "capabilities: frontend.accessibility, frontend.responsive-layout, frontend.visual-design\n",
        )
    );

    let manifest_root = manifest_root.to_string_lossy().into_owned();
    let reindex = command_output(&[
        "registry",
        "reindex",
        "--manifest-root",
        &manifest_root,
        "--index",
        &index,
    ]);
    assert!(reindex.status.success());
    let reindex_stdout = String::from_utf8_lossy(&reindex.stdout);
    assert!(reindex_stdout.contains("indexed: 1\n"));
    assert!(reindex_stdout.contains("excluded: 0\n"));
    assert!(reindex_stdout.contains("fingerprint: fnv1a64:"));
    assert!(reindex_stdout.contains("index: "));
}

#[test]
fn registry_cli_json_outputs_conform_to_wave003_contracts() {
    let (temporary, index) = fixture_index();
    let manifest_root = temporary.path().join("manifests");
    let index_path = Path::new(&index);

    let reindex = reindex_json(&manifest_root, index_path);
    assert_object_keys(
        &reindex,
        &[
            "schema_version",
            "index_schema_version",
            "indexed",
            "excluded",
            "catalog_fingerprint",
        ],
    );
    assert_eq!(reindex["schema_version"], "1.0.0");
    assert_eq!(reindex["index_schema_version"], 1);
    assert_eq!(reindex["indexed"], 1);
    assert_eq!(reindex["excluded"], Value::Array(Vec::new()));
    assert!(
        reindex["catalog_fingerprint"]
            .as_str()
            .is_some_and(
                |fingerprint| fingerprint.starts_with("fnv1a64:") && fingerprint.len() == 24
            )
    );

    let status = json_stdout(&["--format", "json", "registry", "status", "--index", &index]);
    assert_object_keys(
        &status,
        &[
            "schema_version",
            "exists",
            "index_schema_version",
            "compatible",
            "integrity_ok",
            "fts5_available",
            "reindex_required",
            "resource_count",
            "excluded_count",
            "catalog_fingerprint",
            "detail",
        ],
    );
    assert_eq!(status["schema_version"], "1.0.0");
    assert_eq!(status["resource_count"], 1);
    assert_eq!(status["excluded_count"], 0);

    let search = json_stdout(&[
        "--format",
        "json",
        "search",
        "--exact",
        "ossus-example.frontend-review",
        "--index",
        &index,
    ]);
    assert_object_keys(&search, &["schema_version", "results", "limit", "offset"]);
    assert_eq!(search["schema_version"], "1.0.0");
    assert_eq!(search["limit"], 50);
    assert_eq!(search["offset"], 0);
    let results = search["results"]
        .as_array()
        .expect("results should be an array");
    assert_eq!(results.len(), 1);
    assert_resource_output_contract(&results[0]);

    let show = json_stdout(&[
        "--format",
        "json",
        "show",
        "ossus-example.frontend-review",
        "--index",
        &index,
    ]);
    assert_object_keys(&show, &["schema_version", "resource"]);
    assert_eq!(show["schema_version"], "1.0.0");
    assert_resource_output_contract(&show["resource"]);
}

#[test]
fn registry_cli_reindex_is_deterministic_and_excludes_malformed_manifests() {
    let temporary = TempDir::new().expect("temporary directory should be created");
    let manifest_root = temporary.path().join("manifests");
    fs::create_dir(&manifest_root).expect("manifest directory should be created");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../catalog/examples/canonical-manifest.example.toml"),
        manifest_root.join("valid.toml"),
    )
    .expect("valid fixture should be copied");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../ossus-registry/tests/fixtures/invalid/malformed-duplicate-key.toml"),
        manifest_root.join("malformed.toml"),
    )
    .expect("malformed fixture should be copied");
    let index = temporary.path().join("registry.sqlite3");

    let first = reindex_json(&manifest_root, &index);
    let second = reindex_json(&manifest_root, &index);
    assert_eq!(first, second, "reindex output should be deterministic");
    assert_eq!(first["indexed"], 1);
    let excluded = first["excluded"]
        .as_array()
        .expect("excluded should be an array");
    assert_eq!(excluded.len(), 1);
    assert_eq!(
        excluded[0]["diagnostics"][0]["reason_code"],
        "manifest.toml.invalid"
    );
}

#[test]
fn registry_cli_handles_empty_conflicting_and_corrupt_indexes() {
    let empty = TempDir::new().expect("temporary directory should be created");
    let empty_root = empty.path().join("manifests");
    fs::create_dir(&empty_root).expect("manifest directory should be created");
    let empty_index = empty.path().join("empty.sqlite3");
    let empty_reindex = reindex_json(&empty_root, &empty_index);
    assert_eq!(empty_reindex["indexed"], 0);
    assert_eq!(empty_reindex["excluded"], Value::Array(Vec::new()));
    let empty_index_text = empty_index.to_string_lossy().into_owned();
    let empty_search = json_stdout(&["--format", "json", "search", "--index", &empty_index_text]);
    assert_eq!(empty_search["results"], Value::Array(Vec::new()));

    let conflict = TempDir::new().expect("temporary directory should be created");
    let conflict_root = conflict.path().join("manifests");
    fs::create_dir(&conflict_root).expect("manifest directory should be created");
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/examples/canonical-manifest.example.toml");
    fs::copy(&example, conflict_root.join("first.toml")).expect("first fixture should be copied");
    fs::copy(&example, conflict_root.join("second.toml")).expect("second fixture should be copied");
    let conflict_index = conflict.path().join("conflict.sqlite3");
    let conflict_root_text = conflict_root.to_string_lossy().into_owned();
    let conflict_index_text = conflict_index.to_string_lossy().into_owned();
    let conflict_output = command_output(&[
        "--format",
        "json",
        "registry",
        "reindex",
        "--manifest-root",
        &conflict_root_text,
        "--index",
        &conflict_index_text,
    ]);
    assert_eq!(conflict_output.status.code(), Some(20));
    let conflict_json = json_output(&conflict_output);
    assert_eq!(conflict_json["error"], "registry-error");
    assert_eq!(
        conflict_json["conflicts"][0]["reason_code"],
        "registry.resource-id.duplicate"
    );

    let (temporary, index) = fixture_index();
    fs::write(&index, b"not a sqlite database")
        .expect("index corruption fixture should be written");
    let corrupt = command_output(&["--format", "json", "registry", "status", "--index", &index]);
    assert_eq!(corrupt.status.code(), Some(20));
    let corrupt_json = json_output(&corrupt);
    assert_object_keys(
        &corrupt_json,
        &[
            "schema_version",
            "exists",
            "index_schema_version",
            "compatible",
            "integrity_ok",
            "fts5_available",
            "reindex_required",
            "resource_count",
            "excluded_count",
            "catalog_fingerprint",
            "detail",
        ],
    );
    assert_eq!(corrupt_json["reindex_required"], true);
    assert_eq!(corrupt_json["integrity_ok"], false);
    drop(temporary);
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
