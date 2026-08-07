//! Disposable SQLite index over trusted canonical Registry metadata.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    path::{Path, PathBuf},
};

use ossus_core::{CapabilityId, CategoryName, RiskTier, RuntimeRequirement, SourceMode, Surface};
use rusqlite::{
    Connection, OpenFlags, OptionalExtension, Transaction, params, params_from_iter,
    types::Value as SqlValue,
};
use serde::Serialize;

use crate::{Budget, CanonicalManifest, Diagnostic, Taxonomy, load_canonical_manifest_report};

/// Current on-disk index schema. Canonical manifests remain the source of truth.
pub const INDEX_SCHEMA_VERSION: u32 = 1;
/// Version of machine-readable Registry command output.
pub const OUTPUT_SCHEMA_VERSION: &str = "1.0.0";
const DEFAULT_LIMIT: u32 = 50;
const MAX_LIMIT: u32 = 200;
const MAX_OFFSET: u32 = 100_000;
const MAX_QUERY_BYTES: usize = 4_096;

const MIGRATION_1: &str = r#"
CREATE TABLE index_metadata (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) STRICT;
CREATE TABLE resources (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT,
    description TEXT,
    resource_type TEXT NOT NULL,
    source_mode TEXT NOT NULL,
    source_repository TEXT NOT NULL,
    source_commit TEXT NOT NULL,
    source_subpath TEXT NOT NULL,
    tree_hash TEXT NOT NULL,
    risk_tier TEXT NOT NULL,
    risk_ordinal INTEGER NOT NULL CHECK (risk_ordinal BETWEEN 0 AND 5),
    manifest_json TEXT NOT NULL
) STRICT;
CREATE TABLE capabilities (
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    capability TEXT NOT NULL,
    optional INTEGER NOT NULL CHECK (optional IN (0, 1)),
    PRIMARY KEY (resource_id, capability)
) STRICT;
CREATE TABLE categories (
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    category TEXT NOT NULL,
    PRIMARY KEY (resource_id, category)
) STRICT;
CREATE TABLE surfaces (
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    surface TEXT NOT NULL,
    PRIMARY KEY (resource_id, surface)
) STRICT;
CREATE TABLE runtimes (
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    requirement TEXT NOT NULL,
    PRIMARY KEY (resource_id, requirement)
) STRICT;
CREATE VIRTUAL TABLE resource_fts USING fts5(
    resource_id UNINDEXED,
    name,
    description,
    triggers,
    tokenize = 'unicode61'
);
CREATE INDEX resources_name ON resources(name);
CREATE INDEX resources_source ON resources(source_repository, source_commit, source_subpath);
CREATE INDEX capabilities_value ON capabilities(capability, resource_id);
CREATE INDEX categories_value ON categories(category, resource_id);
CREATE INDEX surfaces_value ON surfaces(surface, resource_id);
CREATE INDEX runtimes_value ON runtimes(requirement, resource_id);
PRAGMA user_version = 1;
"#;

/// Errors from Registry indexing and local queries.
#[derive(Debug)]
pub enum RegistryError {
    Io {
        operation: &'static str,
        path: PathBuf,
    },
    Database {
        operation: &'static str,
        message: String,
    },
    InvalidQuery(String),
    UnknownCapability(String),
    Conflict(Vec<RegistryConflict>),
    ReindexRequired(String),
    Serialization(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { operation, path } => write!(
                formatter,
                "{operation} failed for {}",
                ossus_core::escaped_display_value(&path.display().to_string())
            ),
            Self::Database { operation, message } => {
                write!(formatter, "Registry database {operation} failed: {message}")
            }
            Self::InvalidQuery(message) => write!(formatter, "invalid search query: {message}"),
            Self::UnknownCapability(value) => write!(
                formatter,
                "unknown capability `{}`",
                ossus_core::bounded_display_value(value)
            ),
            Self::Conflict(conflicts) => {
                write!(
                    formatter,
                    "Registry rebuild found {} conflict(s)",
                    conflicts.len()
                )
            }
            Self::ReindexRequired(message) => write!(formatter, "reindex required: {message}"),
            Self::Serialization(message) => {
                write!(formatter, "Registry serialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for RegistryError {}

/// Stable, machine-readable conflict discovered before index replacement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RegistryConflict {
    pub reason_code: String,
    pub identity: String,
    pub paths: Vec<String>,
}

/// A malformed canonical file excluded from the rebuilt index.
#[derive(Clone, Debug, Serialize)]
pub struct ExcludedManifest {
    pub path: String,
    pub diagnostics: Vec<Diagnostic>,
}

/// Result of rebuilding the disposable local index.
#[derive(Clone, Debug, Serialize)]
pub struct ReindexReport {
    pub schema_version: &'static str,
    pub index_schema_version: u32,
    pub indexed: usize,
    pub excluded: Vec<ExcludedManifest>,
    pub catalog_fingerprint: String,
}

/// Health and compatibility of an existing local index.
#[derive(Clone, Debug, Serialize)]
pub struct RegistryStatus {
    pub schema_version: &'static str,
    pub exists: bool,
    pub index_schema_version: Option<u32>,
    pub compatible: bool,
    pub integrity_ok: bool,
    pub fts5_available: bool,
    pub reindex_required: bool,
    pub resource_count: u64,
    pub excluded_count: u64,
    pub catalog_fingerprint: Option<String>,
    pub detail: String,
}

/// Bounded trusted-metadata search request.
#[derive(Clone, Debug, Default)]
pub struct SearchQuery {
    pub text: Option<String>,
    pub exact: Option<String>,
    pub capabilities: Vec<String>,
    pub categories: Vec<CategoryName>,
    pub surfaces: Vec<Surface>,
    pub source_modes: Vec<SourceMode>,
    pub runtimes: Vec<RuntimeRequirement>,
    pub maximum_risk: Option<RiskTier>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Trusted canonical metadata returned from the local index.
#[derive(Clone, Debug, Serialize)]
pub struct ResourceRecord {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub resource_type: String,
    pub source_mode: String,
    pub source_repository: String,
    pub source_commit: String,
    pub source_subpath: Option<String>,
    pub tree_hash: String,
    pub capabilities: Vec<String>,
    pub categories: Vec<String>,
    pub surfaces: Vec<String>,
    pub runtimes: Vec<String>,
    pub risk_tier: String,
    pub review_status: String,
    pub distribution_mode: String,
}

/// Versioned search response.
#[derive(Clone, Debug, Serialize)]
pub struct SearchResults {
    pub schema_version: &'static str,
    pub results: Vec<ResourceRecord>,
    pub limit: u32,
    pub offset: u32,
}

/// Versioned show response.
#[derive(Clone, Debug, Serialize)]
pub struct ShowResult {
    pub schema_version: &'static str,
    pub resource: Option<ResourceRecord>,
}

struct LoadedManifest {
    path: PathBuf,
    manifest: CanonicalManifest,
    json: String,
}

/// Rebuilds an index from canonical TOML manifests beneath `manifest_root`.
///
/// Files are sorted before parsing and insertion. A sibling staging database is
/// fully validated and closed before it replaces the live index. A failed build
/// therefore leaves any prior valid index untouched.
pub fn rebuild_index(
    manifest_root: &Path,
    index_path: &Path,
    budget: &Budget,
) -> Result<ReindexReport, RegistryError> {
    let paths = discover_manifests(manifest_root, budget.max_manifests_per_source)?;
    let mut loaded = Vec::new();
    let mut excluded = Vec::new();

    for path in paths {
        match load_canonical_manifest_report(&path, budget) {
            Ok(report) => {
                let json = serde_json::to_string(&report.manifest)
                    .map_err(|error| RegistryError::Serialization(error.to_string()))?;
                loaded.push(LoadedManifest {
                    path,
                    manifest: report.manifest,
                    json,
                });
            }
            Err(diagnostics) => excluded.push(ExcludedManifest {
                path: display_path(&path),
                diagnostics,
            }),
        }
    }

    let conflicts = detect_conflicts(&loaded);
    if !conflicts.is_empty() {
        return Err(RegistryError::Conflict(conflicts));
    }

    let fingerprint = catalog_fingerprint(&loaded);
    let stage_path = sibling_stage_path(index_path);
    remove_if_exists(&stage_path)?;
    let build_result = build_stage(&stage_path, &loaded, excluded.len(), &fingerprint);
    if let Err(error) = build_result {
        let _ = fs::remove_file(&stage_path);
        return Err(error);
    }

    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent).map_err(|_| RegistryError::Io {
            operation: "create index directory",
            path: parent.to_path_buf(),
        })?;
    }
    fs::rename(&stage_path, index_path).map_err(|_| {
        let _ = fs::remove_file(&stage_path);
        RegistryError::Io {
            operation: "replace live index",
            path: index_path.to_path_buf(),
        }
    })?;

    Ok(ReindexReport {
        schema_version: OUTPUT_SCHEMA_VERSION,
        index_schema_version: INDEX_SCHEMA_VERSION,
        indexed: loaded.len(),
        excluded,
        catalog_fingerprint: fingerprint,
    })
}

/// Inspects an index without rebuilding it or accessing any external source.
pub fn registry_status(index_path: &Path) -> RegistryStatus {
    if !index_path.is_file() {
        return RegistryStatus {
            schema_version: OUTPUT_SCHEMA_VERSION,
            exists: false,
            index_schema_version: None,
            compatible: false,
            integrity_ok: false,
            fts5_available: sqlite_has_fts5(),
            reindex_required: true,
            resource_count: 0,
            excluded_count: 0,
            catalog_fingerprint: None,
            detail: String::from("local index does not exist"),
        };
    }

    let connection = match open_read_only(index_path) {
        Ok(connection) => connection,
        Err(error) => return unhealthy_status(Some(0), format!("{error}")),
    };
    let version = connection
        .pragma_query_value(None, "user_version", |row| row.get::<_, u32>(0))
        .ok();
    if version != Some(INDEX_SCHEMA_VERSION) {
        return RegistryStatus {
            schema_version: OUTPUT_SCHEMA_VERSION,
            exists: true,
            index_schema_version: version,
            compatible: false,
            integrity_ok: false,
            fts5_available: sqlite_has_fts5(),
            reindex_required: true,
            resource_count: 0,
            excluded_count: 0,
            catalog_fingerprint: None,
            detail: String::from("unsupported index schema version"),
        };
    }

    let (integrity, integrity_detail) = match read_only_integrity_check(&connection) {
        Ok(()) => (true, None),
        Err(error) => (false, Some(error)),
    };
    let fts = connection
        .query_row(
            "SELECT count(*) FROM resource_fts WHERE resource_fts MATCH ?",
            ["\"__ossus_health_probe__\""],
            |row| row.get::<_, u64>(0),
        )
        .is_ok();
    let count = connection
        .query_row("SELECT count(*) FROM resources", [], |row| {
            row.get::<_, u64>(0)
        })
        .map_err(|error| error.to_string());
    let excluded_count = metadata_value(&connection, "excluded_count").and_then(|value| {
        value
            .parse::<u64>()
            .map_err(|_| String::from("excluded_count metadata is not an unsigned integer"))
    });
    let fingerprint = metadata_value(&connection, "catalog_fingerprint");
    let metadata_ok = excluded_count.is_ok() && fingerprint.is_ok();
    let healthy = integrity && fts && count.is_ok() && metadata_ok;
    let detail = if healthy {
        String::from("local index is healthy")
    } else if let Some(detail) = integrity_detail {
        format!("local index integrity check failed: {detail}")
    } else if let Err(detail) = &count {
        format!("local index resource table is unavailable: {detail}")
    } else if let Err(detail) = &excluded_count {
        format!("local index metadata is invalid: {detail}")
    } else if let Err(detail) = &fingerprint {
        format!("local index metadata is invalid: {detail}")
    } else {
        String::from("local index failed FTS5 checks")
    };

    RegistryStatus {
        schema_version: OUTPUT_SCHEMA_VERSION,
        exists: true,
        index_schema_version: version,
        compatible: true,
        integrity_ok: integrity,
        fts5_available: fts,
        reindex_required: !healthy,
        resource_count: count.unwrap_or(0),
        excluded_count: excluded_count.unwrap_or(0),
        catalog_fingerprint: fingerprint.ok(),
        detail,
    }
}

/// Searches only canonical metadata stored in the disposable local index.
pub fn search_index(
    index_path: &Path,
    query: &SearchQuery,
) -> Result<SearchResults, RegistryError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
    let offset = query.offset.unwrap_or(0);
    validate_query(query, limit, offset)?;
    let taxonomy = Taxonomy::load_builtin(&Budget::default()).map_err(|_| {
        RegistryError::ReindexRequired(String::from("built-in taxonomy could not be loaded"))
    })?;
    let capabilities = normalize_capabilities(&query.capabilities, &taxonomy)?;
    let connection = checked_connection(index_path)?;

    let mut sql = String::from("SELECT r.manifest_json FROM resources r");
    let mut predicates = Vec::new();
    let mut values = Vec::<SqlValue>::new();

    if let Some(text) = query.text.as_deref() {
        let fts_query = safe_fts_query(text)?;
        sql.push_str(" JOIN resource_fts ON resource_fts.resource_id = r.id");
        predicates.push(String::from("resource_fts MATCH ?"));
        values.push(SqlValue::Text(fts_query));
    }
    if let Some(exact) = query.exact.as_deref() {
        predicates.push(String::from("(r.id = ? OR r.name = ? COLLATE NOCASE)"));
        values.push(SqlValue::Text(exact.to_owned()));
        values.push(SqlValue::Text(exact.to_owned()));
    }
    for capability in capabilities {
        predicates.push(String::from(
            "EXISTS (SELECT 1 FROM capabilities c WHERE c.resource_id = r.id AND c.capability = ?)",
        ));
        values.push(SqlValue::Text(capability));
    }
    for category in &query.categories {
        predicates.push(String::from(
            "EXISTS (SELECT 1 FROM categories c WHERE c.resource_id = r.id AND c.category = ?)",
        ));
        values.push(SqlValue::Text(category.as_str().to_owned()));
    }
    for surface in &query.surfaces {
        predicates.push(String::from(
            "EXISTS (SELECT 1 FROM surfaces s WHERE s.resource_id = r.id AND s.surface = ?)",
        ));
        values.push(SqlValue::Text(surface.as_str().to_owned()));
    }
    if !query.source_modes.is_empty() {
        let placeholders = std::iter::repeat_n("?", query.source_modes.len())
            .collect::<Vec<_>>()
            .join(", ");
        predicates.push(format!("r.source_mode IN ({placeholders})"));
        values.extend(
            query
                .source_modes
                .iter()
                .map(|source_mode| SqlValue::Text(source_mode.as_str().to_owned())),
        );
    }
    for runtime in &query.runtimes {
        predicates.push(String::from(
            "EXISTS (SELECT 1 FROM runtimes rt WHERE rt.resource_id = r.id AND rt.requirement = ?)",
        ));
        values.push(SqlValue::Text(runtime.as_str().to_owned()));
    }
    if let Some(risk) = query.maximum_risk {
        predicates.push(String::from("r.risk_ordinal <= ?"));
        values.push(SqlValue::Integer(i64::from(risk_ordinal(risk))));
    }
    if !predicates.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }
    if query.text.is_some() {
        sql.push_str(" ORDER BY bm25(resource_fts), r.id");
    } else {
        sql.push_str(" ORDER BY r.id");
    }
    sql.push_str(" LIMIT ? OFFSET ?");
    values.push(SqlValue::Integer(i64::from(limit)));
    values.push(SqlValue::Integer(i64::from(offset)));

    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| database_error("prepare search", error))?;
    let rows = statement
        .query_map(params_from_iter(values), |row| row.get::<_, String>(0))
        .map_err(|error| database_error("execute search", error))?;
    let mut results = Vec::new();
    for row in rows {
        let json = row.map_err(|error| database_error("read search result", error))?;
        let manifest: CanonicalManifest = serde_json::from_str(&json)
            .map_err(|error| RegistryError::ReindexRequired(error.to_string()))?;
        results.push(resource_record(&manifest));
    }

    Ok(SearchResults {
        schema_version: OUTPUT_SCHEMA_VERSION,
        results,
        limit,
        offset,
    })
}

/// Returns one exact resource ID from the index without rebuilding it.
pub fn show_resource(index_path: &Path, id: &str) -> Result<ShowResult, RegistryError> {
    let connection = checked_connection(index_path)?;
    let json = connection
        .query_row(
            "SELECT manifest_json FROM resources WHERE id = ?",
            [id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| database_error("show resource", error))?;
    let resource = match json {
        Some(json) => {
            let manifest: CanonicalManifest = serde_json::from_str(&json)
                .map_err(|error| RegistryError::ReindexRequired(error.to_string()))?;
            Some(resource_record(&manifest))
        }
        None => None,
    };
    Ok(ShowResult {
        schema_version: OUTPUT_SCHEMA_VERSION,
        resource,
    })
}

fn discover_manifests(root: &Path, maximum: usize) -> Result<Vec<PathBuf>, RegistryError> {
    if !root.is_dir() {
        return Err(RegistryError::Io {
            operation: "read manifest directory",
            path: root.to_path_buf(),
        });
    }
    let mut pending = vec![root.to_path_buf()];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory).map_err(|_| RegistryError::Io {
            operation: "read manifest directory",
            path: directory.clone(),
        })?;
        for entry in entries {
            let entry = entry.map_err(|_| RegistryError::Io {
                operation: "read manifest directory entry",
                path: directory.clone(),
            })?;
            let file_type = entry.file_type().map_err(|_| RegistryError::Io {
                operation: "inspect manifest path",
                path: entry.path(),
            })?;
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "toml")
            {
                paths.push(entry.path());
                if paths.len() > maximum {
                    return Err(RegistryError::InvalidQuery(format!(
                        "manifest count exceeds configured limit of {maximum}"
                    )));
                }
            }
        }
    }
    paths.sort();
    Ok(paths)
}

fn detect_conflicts(loaded: &[LoadedManifest]) -> Vec<RegistryConflict> {
    let mut ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut sources: BTreeMap<(String, String, String), BTreeMap<String, Vec<String>>> =
        BTreeMap::new();
    for item in loaded {
        let path = display_path(&item.path);
        ids.entry(item.manifest.id.as_str().to_owned())
            .or_default()
            .push(path.clone());
        let identity = (
            item.manifest.source.repository.clone(),
            item.manifest.source.commit.as_str().to_owned(),
            item.manifest.source.subpath.clone().unwrap_or_default(),
        );
        sources
            .entry(identity)
            .or_default()
            .entry(item.manifest.source.tree_hash.as_str().to_owned())
            .or_default()
            .push(path);
    }

    let mut conflicts = Vec::new();
    for (id, mut paths) in ids {
        if paths.len() > 1 {
            paths.sort();
            conflicts.push(RegistryConflict {
                reason_code: String::from("registry.resource-id.duplicate"),
                identity: id,
                paths,
            });
        }
    }
    for ((repository, commit, subpath), hashes) in sources {
        if hashes.len() > 1 {
            let mut paths = hashes.into_values().flatten().collect::<Vec<_>>();
            paths.sort();
            conflicts.push(RegistryConflict {
                reason_code: String::from("registry.source-identity.hash-conflict"),
                identity: format!("{repository}@{commit}:{subpath}"),
                paths,
            });
        }
    }
    conflicts.sort_by(|left, right| {
        (&left.reason_code, &left.identity).cmp(&(&right.reason_code, &right.identity))
    });
    conflicts
}

fn build_stage(
    path: &Path,
    loaded: &[LoadedManifest],
    excluded_count: usize,
    fingerprint: &str,
) -> Result<(), RegistryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| RegistryError::Io {
            operation: "create staging directory",
            path: parent.to_path_buf(),
        })?;
    }
    let mut connection =
        Connection::open(path).map_err(|error| database_error("open stage", error))?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = DELETE;")
        .map_err(|error| database_error("configure stage", error))?;
    migrate(&mut connection)?;
    let transaction = connection
        .transaction()
        .map_err(|error| database_error("begin rebuild", error))?;
    transaction
        .execute(
            "INSERT INTO index_metadata(key, value) VALUES ('catalog_fingerprint', ?)",
            [fingerprint],
        )
        .map_err(|error| database_error("write metadata", error))?;
    transaction
        .execute(
            "INSERT INTO index_metadata(key, value) VALUES ('excluded_count', ?)",
            [excluded_count.to_string()],
        )
        .map_err(|error| database_error("write metadata", error))?;
    for item in loaded {
        insert_manifest(&transaction, item)?;
    }
    transaction
        .commit()
        .map_err(|error| database_error("commit rebuild", error))?;

    let foreign_key_errors = connection
        .query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |row| {
            row.get::<_, u64>(0)
        })
        .map_err(|error| database_error("check foreign keys", error))?;
    let integrity =
        integrity_is_ok(&connection).map_err(|error| database_error("check integrity", error))?;
    connection
        .query_row("SELECT count(*) FROM resource_fts", [], |row| {
            row.get::<_, u64>(0)
        })
        .map_err(|error| database_error("verify FTS5", error))?;
    if foreign_key_errors != 0 || !integrity {
        return Err(RegistryError::ReindexRequired(String::from(
            "staging index failed integrity checks",
        )));
    }
    drop(connection);
    Ok(())
}

fn migrate(connection: &mut Connection) -> Result<(), RegistryError> {
    let version = connection
        .pragma_query_value(None, "user_version", |row| row.get::<_, u32>(0))
        .map_err(|error| database_error("read schema version", error))?;
    match version {
        0 => connection
            .execute_batch(MIGRATION_1)
            .map_err(|error| database_error("apply migration 1", error)),
        INDEX_SCHEMA_VERSION => Ok(()),
        other => Err(RegistryError::ReindexRequired(format!(
            "unsupported index schema version {other}"
        ))),
    }
}

fn insert_manifest(
    transaction: &Transaction<'_>,
    item: &LoadedManifest,
) -> Result<(), RegistryError> {
    let manifest = &item.manifest;
    transaction
        .execute(
            "INSERT INTO resources (
                id, name, description, resource_type, source_mode, source_repository,
                source_commit, source_subpath, tree_hash, risk_tier, risk_ordinal, manifest_json
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                manifest.id.as_str(),
                manifest.name,
                manifest.description,
                manifest.resource_type.as_str(),
                manifest.source.mode.as_str(),
                manifest.source.repository,
                manifest.source.commit.as_str(),
                manifest.source.subpath.as_deref().unwrap_or(""),
                manifest.source.tree_hash.as_str(),
                manifest.risk.tier.as_str(),
                risk_ordinal(manifest.risk.tier),
                item.json,
            ],
        )
        .map_err(|error| database_error("insert resource", error))?;

    for capability in &manifest.capabilities.required {
        insert_relation(
            transaction,
            "INSERT INTO capabilities(resource_id, capability, optional) VALUES (?, ?, 0)",
            manifest.id.as_str(),
            capability.as_str(),
        )?;
    }
    if let Some(optional) = &manifest.capabilities.optional {
        for capability in optional {
            insert_relation(
                transaction,
                "INSERT INTO capabilities(resource_id, capability, optional) VALUES (?, ?, 1)",
                manifest.id.as_str(),
                capability.as_str(),
            )?;
        }
    }
    for category in &manifest.categories {
        insert_relation(
            transaction,
            "INSERT INTO categories(resource_id, category) VALUES (?, ?)",
            manifest.id.as_str(),
            category.as_str(),
        )?;
    }
    for surface in &manifest.compatibility.surfaces {
        insert_relation(
            transaction,
            "INSERT INTO surfaces(resource_id, surface) VALUES (?, ?)",
            manifest.id.as_str(),
            surface.as_str(),
        )?;
    }
    for runtime in &manifest.runtime.requirements {
        insert_relation(
            transaction,
            "INSERT INTO runtimes(resource_id, requirement) VALUES (?, ?)",
            manifest.id.as_str(),
            runtime.as_str(),
        )?;
    }
    transaction
        .execute(
            "INSERT INTO resource_fts(resource_id, name, description, triggers) VALUES (?, ?, ?, ?)",
            params![
                manifest.id.as_str(),
                manifest.name.as_deref().unwrap_or(""),
                manifest.description.as_deref().unwrap_or(""),
                manifest.triggers.as_ref().map(|items| items.join(" ")).unwrap_or_default(),
            ],
        )
        .map_err(|error| database_error("insert FTS metadata", error))?;
    Ok(())
}

fn insert_relation(
    transaction: &Transaction<'_>,
    sql: &str,
    resource_id: &str,
    value: &str,
) -> Result<(), RegistryError> {
    transaction
        .execute(sql, params![resource_id, value])
        .map_err(|error| database_error("insert relation", error))?;
    Ok(())
}

fn validate_query(query: &SearchQuery, limit: u32, offset: u32) -> Result<(), RegistryError> {
    if limit == 0 || limit > MAX_LIMIT {
        return Err(RegistryError::InvalidQuery(format!(
            "limit must be between 1 and {MAX_LIMIT}"
        )));
    }
    if offset > MAX_OFFSET {
        return Err(RegistryError::InvalidQuery(format!(
            "offset must not exceed {MAX_OFFSET}"
        )));
    }
    for value in [query.text.as_deref(), query.exact.as_deref()]
        .into_iter()
        .flatten()
    {
        if value.len() > MAX_QUERY_BYTES {
            return Err(RegistryError::InvalidQuery(format!(
                "query text exceeds {MAX_QUERY_BYTES} bytes"
            )));
        }
    }
    Ok(())
}

fn normalize_capabilities(
    values: &[String],
    taxonomy: &Taxonomy,
) -> Result<Vec<String>, RegistryError> {
    let mut normalized = BTreeSet::new();
    for value in values {
        if let Ok(id) = CapabilityId::parse(value)
            && taxonomy.capability(&id).is_some()
        {
            normalized.insert(id.as_str().to_owned());
            continue;
        }
        if let Some(id) = taxonomy.alias_target(value) {
            normalized.insert(id.as_str().to_owned());
            continue;
        }
        return Err(RegistryError::UnknownCapability(value.clone()));
    }
    Ok(normalized.into_iter().collect())
}

fn safe_fts_query(value: &str) -> Result<String, RegistryError> {
    let terms = value
        .split_whitespace()
        .filter(|term| !term.is_empty())
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>();
    if terms.is_empty() {
        return Err(RegistryError::InvalidQuery(String::from(
            "search text must contain a non-whitespace term",
        )));
    }
    Ok(terms.join(" AND "))
}

fn checked_connection(path: &Path) -> Result<Connection, RegistryError> {
    let status = registry_status(path);
    if status.reindex_required {
        return Err(RegistryError::ReindexRequired(status.detail));
    }
    open_read_only(path)
}

fn open_read_only(path: &Path) -> Result<Connection, RegistryError> {
    let connection = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|error| database_error("open read-only", error))?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|error| database_error("configure read-only", error))?;
    Ok(connection)
}

fn resource_record(manifest: &CanonicalManifest) -> ResourceRecord {
    let mut capabilities = manifest
        .capabilities
        .required
        .iter()
        .map(|value| value.as_str().to_owned())
        .collect::<Vec<_>>();
    if let Some(optional) = &manifest.capabilities.optional {
        capabilities.extend(optional.iter().map(|value| value.as_str().to_owned()));
    }
    capabilities.sort();
    ResourceRecord {
        id: manifest.id.as_str().to_owned(),
        name: manifest.name.clone(),
        description: manifest.description.clone(),
        resource_type: manifest.resource_type.as_str().to_owned(),
        source_mode: manifest.source.mode.as_str().to_owned(),
        source_repository: manifest.source.repository.clone(),
        source_commit: manifest.source.commit.as_str().to_owned(),
        source_subpath: manifest.source.subpath.clone(),
        tree_hash: manifest.source.tree_hash.as_str().to_owned(),
        capabilities,
        categories: sorted_strings(manifest.categories.iter().map(|value| value.as_str())),
        surfaces: sorted_strings(
            manifest
                .compatibility
                .surfaces
                .iter()
                .map(|value| value.as_str()),
        ),
        runtimes: sorted_strings(
            manifest
                .runtime
                .requirements
                .iter()
                .map(|value| value.as_str()),
        ),
        risk_tier: manifest.risk.tier.as_str().to_owned(),
        review_status: manifest.review.status.as_str().to_owned(),
        distribution_mode: manifest.distribution.mode.as_str().to_owned(),
    }
}

fn sorted_strings<'a>(items: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut values = items.map(ToOwned::to_owned).collect::<Vec<_>>();
    values.sort();
    values
}

fn risk_ordinal(risk: RiskTier) -> u8 {
    match risk {
        RiskTier::R0 => 0,
        RiskTier::R1 => 1,
        RiskTier::R2 => 2,
        RiskTier::R3 => 3,
        RiskTier::R4 => 4,
        RiskTier::R5 => 5,
    }
}

fn catalog_fingerprint(loaded: &[LoadedManifest]) -> String {
    // FNV-1a is used as a deterministic change fingerprint, not as a content
    // authenticity hash. Immutable source authenticity remains `tree_hash`.
    let mut state = 0xcbf29ce484222325_u64;
    for item in loaded {
        for byte in item.json.as_bytes().iter().chain(std::iter::once(&0_u8)) {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x100000001b3);
        }
    }
    format!("fnv1a64:{state:016x}")
}

fn sibling_stage_path(index_path: &Path) -> PathBuf {
    let file_name = index_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("registry.sqlite3");
    index_path.with_file_name(format!(".{file_name}.reindex-stage"))
}

fn remove_if_exists(path: &Path) -> Result<(), RegistryError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(RegistryError::Io {
            operation: "remove stale staging index",
            path: path.to_path_buf(),
        }),
    }
}

fn read_only_integrity_check(connection: &Connection) -> Result<(), String> {
    for table in [
        "index_metadata",
        "resources",
        "capabilities",
        "categories",
        "surfaces",
        "runtimes",
    ] {
        let sql = format!("PRAGMA quick_check('{table}')");
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| error.to_string())?;
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|error| error.to_string())?;
        for row in rows {
            let result = row.map_err(|error| error.to_string())?;
            if result != "ok" {
                return Err(result);
            }
        }
    }
    let foreign_key_errors = connection
        .query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |row| {
            row.get::<_, u64>(0)
        })
        .map_err(|error| error.to_string())?;
    if foreign_key_errors != 0 {
        return Err(format!(
            "foreign-key check found {foreign_key_errors} violation(s)"
        ));
    }
    Ok(())
}

fn integrity_is_ok(connection: &Connection) -> Result<bool, rusqlite::Error> {
    connection
        .query_row("PRAGMA quick_check", [], |row| row.get::<_, String>(0))
        .map(|value| value == "ok")
}

fn metadata_value(connection: &Connection, key: &str) -> Result<String, String> {
    connection
        .query_row(
            "SELECT value FROM index_metadata WHERE key = ?",
            [key],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("required metadata `{key}` is missing"))
}

fn sqlite_has_fts5() -> bool {
    Connection::open_in_memory()
        .and_then(|connection| {
            connection.execute_batch("CREATE VIRTUAL TABLE fts_probe USING fts5(value);")
        })
        .is_ok()
}

fn unhealthy_status(version: Option<u32>, detail: String) -> RegistryStatus {
    RegistryStatus {
        schema_version: OUTPUT_SCHEMA_VERSION,
        exists: true,
        index_schema_version: version,
        compatible: false,
        integrity_ok: false,
        fts5_available: sqlite_has_fts5(),
        reindex_required: true,
        resource_count: 0,
        excluded_count: 0,
        catalog_fingerprint: None,
        detail,
    }
}

fn database_error(operation: &'static str, error: rusqlite::Error) -> RegistryError {
    RegistryError::Database {
        operation,
        message: error.to_string(),
    }
}

fn display_path(path: &Path) -> String {
    ossus_core::escaped_display_value(&path.display().to_string())
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        reason = "Registry tests use deterministic local fixtures"
    )]

    use std::{fs, path::Path};

    use ossus_core::{CategoryName, RiskTier, RuntimeRequirement, SourceMode, Surface};
    use rusqlite::Connection;
    use tempfile::TempDir;

    use super::{
        Budget, RegistryError, SearchQuery, rebuild_index, registry_status, search_index,
        show_resource,
    };

    const FIXTURE_NAME: &str = "canonical-manifest.example.toml";

    fn example_manifest() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../catalog/examples")
                .join(FIXTURE_NAME),
        )
        .expect("example manifest should be readable")
    }

    fn fixture_paths() -> (TempDir, std::path::PathBuf, std::path::PathBuf) {
        let directory = TempDir::new().expect("temporary directory should be created");
        let manifests = directory.path().join("manifests");
        fs::create_dir(&manifests).expect("manifest directory should be created");
        let index = directory.path().join("registry.sqlite3");
        (directory, manifests, index)
    }

    fn write_manifest(root: &Path, name: &str, contents: &str) {
        fs::write(root.join(name), contents).expect("fixture manifest should be written");
    }

    #[test]
    fn rebuild_is_deterministic_and_searchable() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());

        let first = rebuild_index(&manifests, &index, &Budget::default())
            .expect("first rebuild should succeed");
        let first_bytes = fs::read(&index).expect("first index should be readable");
        let second = rebuild_index(&manifests, &index, &Budget::default())
            .expect("second rebuild should succeed");
        let second_bytes = fs::read(&index).expect("second index should be readable");

        assert_eq!(first.catalog_fingerprint, second.catalog_fingerprint);
        assert_eq!(first.indexed, 1);
        assert_eq!(first_bytes, second_bytes);
        let status = registry_status(&index);
        assert!(status.integrity_ok);
        assert!(status.fts5_available);
        assert!(!status.reindex_required);

        let results = search_index(
            &index,
            &SearchQuery {
                text: Some(String::from("responsive layout")),
                capabilities: vec![String::from("frontend.responsive-layout")],
                categories: vec![CategoryName::parse("frontend").unwrap()],
                surfaces: vec![Surface::try_from("claude-code-cli").unwrap()],
                source_modes: vec![SourceMode::try_from("remote-index").unwrap()],
                runtimes: vec![RuntimeRequirement::try_from("instruction-only").unwrap()],
                maximum_risk: Some(RiskTier::R0),
                ..SearchQuery::default()
            },
        )
        .expect("combined search should succeed");
        assert_eq!(results.results.len(), 1);
        assert_eq!(results.results[0].id, "ossus-example.frontend-review");

        let shown = show_resource(&index, "ossus-example.frontend-review")
            .expect("show should succeed")
            .resource
            .expect("resource should exist");
        assert_eq!(shown.id, "ossus-example.frontend-review");
        assert!(
            show_resource(&index, "ossus-example.missing")
                .expect("missing show should be a successful lookup")
                .resource
                .is_none()
        );
    }

    #[test]
    fn exact_alias_and_unknown_capability_searches_are_explicit() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        for exact in ["ossus-example.frontend-review", "frontend review"] {
            let results = search_index(
                &index,
                &SearchQuery {
                    exact: Some(String::from(exact)),
                    ..SearchQuery::default()
                },
            )
            .expect("exact ID or case-insensitive name should succeed");
            assert_eq!(results.results.len(), 1);
        }
        let aliased = search_index(
            &index,
            &SearchQuery {
                capabilities: vec![String::from("responsive")],
                ..SearchQuery::default()
            },
        )
        .expect("known aliases should resolve to canonical capabilities");
        assert_eq!(aliased.results.len(), 1);

        let unknown = search_index(
            &index,
            &SearchQuery {
                capabilities: vec![String::from("missing-capability")],
                ..SearchQuery::default()
            },
        )
        .expect_err("unknown capabilities should fail explicitly");
        assert!(matches!(unknown, RegistryError::UnknownCapability(_)));
    }

    #[test]
    fn repeated_source_modes_use_or_semantics() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        let results = search_index(
            &index,
            &SearchQuery {
                source_modes: vec![
                    SourceMode::try_from("remote-index").unwrap(),
                    SourceMode::try_from("vendored").unwrap(),
                ],
                ..SearchQuery::default()
            },
        )
        .expect("multi-value source filter should succeed");
        assert_eq!(results.results.len(), 1);
    }

    #[test]
    fn fts_metacharacters_are_treated_as_terms() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        let results = search_index(
            &index,
            &SearchQuery {
                text: Some(String::from("responsive OR * NEAR(backend) \"")),
                ..SearchQuery::default()
            },
        )
        .expect("hostile FTS syntax should not become query syntax");
        assert!(results.results.is_empty());
    }

    #[test]
    fn malformed_manifests_are_excluded() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        write_manifest(&manifests, "malformed.toml", "unknown = true\n");

        let report = rebuild_index(&manifests, &index, &Budget::default())
            .expect("rebuild should exclude malformed input");
        assert_eq!(report.indexed, 1);
        assert_eq!(report.excluded.len(), 1);
        assert!(!report.excluded[0].diagnostics.is_empty());
        assert_eq!(registry_status(&index).excluded_count, 1);
    }

    #[test]
    fn duplicate_resource_ids_fail_closed_and_preserve_index() {
        let (_directory, manifests, index) = fixture_paths();
        let fixture = example_manifest();
        write_manifest(&manifests, "first.toml", &fixture);
        rebuild_index(&manifests, &index, &Budget::default()).expect("baseline should rebuild");
        let baseline = fs::read(&index).expect("baseline index should be readable");
        write_manifest(&manifests, "second.toml", &fixture);

        let error = rebuild_index(&manifests, &index, &Budget::default())
            .expect_err("duplicate ID should fail closed");
        let RegistryError::Conflict(conflicts) = error else {
            panic!("expected a conflict error");
        };
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].reason_code, "registry.resource-id.duplicate");
        assert_eq!(fs::read(&index).unwrap(), baseline);
        assert!(!registry_status(&index).reindex_required);
    }

    #[test]
    fn commit_case_collision_with_incompatible_hashes_fails_closed() {
        let (_directory, manifests, index) = fixture_paths();
        let first = example_manifest().replace(
            "commit = \"0123456789abcdef0123456789abcdef01234567\"",
            "commit = \"0123456789abcdef0123456789abcdef01234567\"\n# observed-commit: 0123456789abcdef0123456789abcdef01234567",
        );
        let second = example_manifest()
            .replace(
                "id = \"ossus-example.frontend-review\"",
                "id = \"ossus-example.frontend-review-copy\"",
            )
            .replace(
                "commit = \"0123456789abcdef0123456789abcdef01234567\"",
                "commit = \"0123456789abcdef0123456789abcdef01234567\"\n# observed-commit: 0123456789ABCDEF0123456789ABCDEF01234567",
            )
            .replace(
                "tree_hash = \"sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"",
                "tree_hash = \"sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"",
            );
        write_manifest(&manifests, "lower.toml", &first);
        write_manifest(&manifests, "upper-observed.toml", &second);

        let error = rebuild_index(&manifests, &index, &Budget::default())
            .expect_err("case-normalized source conflict should fail closed");
        let RegistryError::Conflict(conflicts) = error else {
            panic!("expected a conflict error");
        };
        assert_eq!(conflicts.len(), 1);
        assert_eq!(
            conflicts[0].reason_code,
            "registry.source-identity.hash-conflict"
        );
    }

    #[test]
    fn corruption_and_future_schema_require_reindex() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        let connection = Connection::open(&index).expect("index should open");
        connection
            .pragma_update(None, "user_version", 999_u32)
            .expect("schema version should update");
        drop(connection);
        let future = registry_status(&index);
        assert!(!future.compatible);
        assert!(future.reindex_required);

        fs::write(&index, b"not a sqlite database").expect("index should be corrupted");
        let corrupt = registry_status(&index);
        assert!(corrupt.reindex_required);
        assert!(!corrupt.integrity_ok);
    }

    #[test]
    fn missing_required_metadata_requires_reindex() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        let connection = Connection::open(&index).expect("index should open");
        connection
            .execute(
                "DELETE FROM index_metadata WHERE key = 'catalog_fingerprint'",
                [],
            )
            .expect("metadata should be removable for the corruption fixture");
        drop(connection);

        let status = registry_status(&index);
        assert!(status.reindex_required);
        assert!(status.catalog_fingerprint.is_none());
        assert!(status.detail.contains("required metadata"));
    }

    #[test]
    fn query_bounds_are_enforced() {
        let (_directory, manifests, index) = fixture_paths();
        write_manifest(&manifests, FIXTURE_NAME, &example_manifest());
        rebuild_index(&manifests, &index, &Budget::default()).expect("rebuild should succeed");

        for query in [
            SearchQuery {
                limit: Some(201),
                ..SearchQuery::default()
            },
            SearchQuery {
                offset: Some(100_001),
                ..SearchQuery::default()
            },
            SearchQuery {
                text: Some("x".repeat(4_097)),
                ..SearchQuery::default()
            },
            SearchQuery {
                text: Some(String::from(" \t\n")),
                ..SearchQuery::default()
            },
        ] {
            let error = search_index(&index, &query).expect_err("invalid bounds should fail");
            assert!(matches!(error, RegistryError::InvalidQuery(_)));
        }
    }
}
