#![allow(
    clippy::expect_used,
    reason = "release verification must fail immediately when its isolated fixture cannot run"
)]

use std::fs;

use ossus_registry::{Budget, SearchQuery, rebuild_index, registry_status, search_index};

const EXAMPLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/examples/canonical-manifest.example.toml"
));

#[test]
fn bundled_release_sqlite_supports_fts5_queries() {
    let temporary = tempfile::tempdir().expect("create isolated FTS5 test directory");
    let manifests = temporary.path().join("manifests");
    let index = temporary.path().join("registry.sqlite3");
    fs::create_dir(&manifests).expect("create manifest directory");
    fs::write(manifests.join("resource.toml"), EXAMPLE).expect("write fixture manifest");

    let report = rebuild_index(&manifests, &index, &Budget::default())
        .expect("bundled SQLite must create the FTS5 index");
    assert_eq!(report.indexed, 1);
    assert!(registry_status(&index).fts5_available);

    let response = search_index(
        &index,
        &SearchQuery {
            text: Some(String::from("responsive")),
            ..SearchQuery::default()
        },
    )
    .expect("bundled SQLite must execute an FTS5 MATCH query");
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "ossus-example.frontend-review");
}
