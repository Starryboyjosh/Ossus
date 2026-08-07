#![forbid(unsafe_code)]

use std::{
    env,
    path::{Path, PathBuf},
    process::ExitCode,
};

use ossus_core::{
    CURRENT_WAVE, CategoryName, DOMAINS, PRODUCT_NAME, RiskTier, RuntimeRequirement, SourceMode,
    Surface,
};
use ossus_registry::{
    Budget, Diagnostic, DiagnosticClass, DiagnosticSeverity, RegistryError, SearchQuery,
    load_canonical_manifest_report, rebuild_index, registry_status, search_index, show_resource,
};
use serde::Serialize;

const EXIT_USAGE: u8 = 2;
const EXIT_SCHEMA: u8 = 11;
const EXIT_TAXONOMY: u8 = 12;
const EXIT_REGISTRY: u8 = 20;
const EXIT_NOT_IMPLEMENTED: u8 = 69;
const EXIT_INTERNAL: u8 = 70;
const DEFAULT_INDEX_PATH: &str = ".ossus/registry.sqlite3";
const DEFAULT_MANIFEST_ROOT: &str = "catalog/official/manifests";

fn main() -> ExitCode {
    run(env::args().skip(1))
}

fn run<I>(mut args: I) -> ExitCode
where
    I: Iterator<Item = String>,
{
    let arguments: Vec<String> = args.by_ref().collect();
    let (format, arguments) = match parse_global_options(arguments) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("ossus: {message}");
            return ExitCode::from(EXIT_USAGE);
        }
    };
    let Some(command) = arguments.first() else {
        print_help();
        return ExitCode::SUCCESS;
    };

    match command.as_str() {
        "-h" | "--help" | "help" => {
            print_help();
            ExitCode::SUCCESS
        }
        "-V" | "--version" | "version" => {
            println!("ossus {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "status" => {
            print_status();
            ExitCode::SUCCESS
        }
        "plan" => {
            println!("Implementation plan: docs/implementation/06-waves/WAVE_INDEX.md");
            println!("Current gate: {CURRENT_WAVE}");
            ExitCode::SUCCESS
        }
        "validate" => validate_command(&arguments[1..], format),
        "registry" => registry_command(&arguments[1..], format),
        "search" => search_command(&arguments[1..], format),
        "show" => show_command(&arguments[1..], format),
        "init" | "config" | "scan" | "resolve" | "explain" | "activate" | "deactivate" | "lock"
        | "doctor" | "eval" | "audit" | "research" => {
            eprintln!(
                "ossus: command '{command}' is planned but not implemented in the repository scaffold"
            );
            eprintln!("see docs/implementation/06-waves/WAVE_INDEX.md");
            ExitCode::from(EXIT_NOT_IMPLEMENTED)
        }
        _ => {
            eprintln!("ossus: unknown command '{command}'");
            eprintln!("run 'ossus --help' for the bootstrap command list");
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn print_help() {
    println!("{PRODUCT_NAME} — local-first agent capability Registry and Resolver");
    println!();
    println!("USAGE:");
    println!("    ossus <COMMAND>");
    println!();
    println!("BOOTSTRAP COMMANDS:");
    println!("    help       Show this help");
    println!("    version    Show the scaffold package version");
    println!("    status     Show domain and implementation status");
    println!("    plan       Show the implementation-plan entry point");
    println!();
    println!("REGISTRY COMMANDS:");
    println!("    validate   Validate canonical manifest files");
    println!("    registry   Inspect or rebuild the local metadata index");
    println!("    search     Search canonical Registry metadata");
    println!("    show       Show one indexed resource");
    println!();
    println!("PLANNED COMMAND GROUPS:");
    println!("    init config scan resolve explain activate deactivate");
    println!("    lock doctor eval audit research");
}

fn print_validate_help() {
    println!("USAGE:");
    println!("    ossus validate <PATH>...");
    println!();
    println!("Validates one or more canonical TOML manifests with bounded parsing.");
    println!("Use --format json for versioned machine-readable diagnostics.");
}

fn print_status() {
    println!("product: {PRODUCT_NAME}");
    println!("current_wave: {CURRENT_WAVE}");
    println!("repository_state: implementation");
    println!("domains: {}", DOMAINS.join(", "));
    println!("registry: {}", ossus_registry::component_state());
    println!("resolver: {}", ossus_resolver::component_state());
    println!("policy: {}", ossus_policy::component_state());
    println!(
        "claude_adapter: {}",
        ossus_adapter_claude::component_state()
    );
    println!("evaluation: {}", ossus_eval::component_state());
    println!("researcher: planned");
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Human,
    Json,
}

fn parse_global_options(arguments: Vec<String>) -> Result<(OutputFormat, Vec<String>), String> {
    let mut format = OutputFormat::Human;
    let mut remaining = Vec::with_capacity(arguments.len());
    let mut index = 0_usize;
    while index < arguments.len() {
        let argument = &arguments[index];
        if argument == "--format" {
            let Some(value) = arguments.get(index + 1) else {
                return Err(String::from("--format requires 'human' or 'json'"));
            };
            format = parse_format(value)?;
            index += 2;
        } else if let Some(value) = argument.strip_prefix("--format=") {
            format = parse_format(value)?;
            index += 1;
        } else {
            remaining.push(argument.clone());
            index += 1;
        }
    }
    Ok((format, remaining))
}

fn parse_format(value: &str) -> Result<OutputFormat, String> {
    match value {
        "human" => Ok(OutputFormat::Human),
        "json" => Ok(OutputFormat::Json),
        _ => Err(format!(
            "invalid --format value `{}`; expected 'human' or 'json'",
            bounded_for_cli(value)
        )),
    }
}

#[derive(Serialize)]
struct ErrorOutput<'a> {
    schema_version: &'static str,
    error: &'a str,
    message: String,
    conflicts: &'a [ossus_registry::RegistryConflict],
}

fn registry_command(arguments: &[String], format: OutputFormat) -> ExitCode {
    let Some(action) = arguments.first() else {
        eprintln!("ossus registry: expected 'status' or 'reindex'");
        return ExitCode::from(EXIT_USAGE);
    };
    match action.as_str() {
        "status" => {
            let index_path =
                match parse_single_path_option(&arguments[1..], "--index", DEFAULT_INDEX_PATH) {
                    Ok(path) => path,
                    Err(message) => return usage_error("registry status", &message),
                };
            let status = registry_status(&index_path);
            match format {
                OutputFormat::Json => print_serialized(&status),
                OutputFormat::Human => {
                    println!("index: {}", normalized_path(&index_path));
                    println!("status: {}", status.detail);
                    println!(
                        "schema_version: {}",
                        status
                            .index_schema_version
                            .map_or_else(|| String::from("none"), |value| value.to_string())
                    );
                    println!("resources: {}", status.resource_count);
                    println!("excluded: {}", status.excluded_count);
                    println!(
                        "fts5: {}",
                        if status.fts5_available {
                            "available"
                        } else {
                            "unavailable"
                        }
                    );
                    println!("reindex_required: {}", status.reindex_required);
                }
            }
            if status.reindex_required {
                ExitCode::from(EXIT_REGISTRY)
            } else {
                ExitCode::SUCCESS
            }
        }
        "reindex" => {
            let (manifest_root, index_path) = match parse_reindex_options(&arguments[1..]) {
                Ok(paths) => paths,
                Err(message) => return usage_error("registry reindex", &message),
            };
            match rebuild_index(&manifest_root, &index_path, &Budget::default()) {
                Ok(report) => {
                    match format {
                        OutputFormat::Json => print_serialized(&report),
                        OutputFormat::Human => {
                            println!("indexed: {}", report.indexed);
                            println!("excluded: {}", report.excluded.len());
                            println!("fingerprint: {}", report.catalog_fingerprint);
                            println!("index: {}", normalized_path(&index_path));
                        }
                    }
                    ExitCode::SUCCESS
                }
                Err(error) => registry_error("registry reindex", &error, format),
            }
        }
        "-h" | "--help" => {
            println!("USAGE:");
            println!("    ossus registry status [--index PATH]");
            println!("    ossus registry reindex [--manifest-root PATH] [--index PATH]");
            ExitCode::SUCCESS
        }
        _ => usage_error("registry", "expected 'status' or 'reindex'"),
    }
}

fn search_command(arguments: &[String], format: OutputFormat) -> ExitCode {
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        println!("USAGE:");
        println!("    ossus search [TEXT] [FILTERS] [--index PATH]");
        println!();
        println!("FILTERS:");
        println!("    --exact VALUE --capability ID --category NAME --surface SURFACE");
        println!("    --source-mode MODE --runtime REQUIREMENT --risk-max TIER");
        println!("    --limit N --offset N");
        return ExitCode::SUCCESS;
    }
    let (index_path, query) = match parse_search_options(arguments) {
        Ok(parsed) => parsed,
        Err(message) => return usage_error("search", &message),
    };
    match search_index(&index_path, &query) {
        Ok(response) => {
            match format {
                OutputFormat::Json => print_serialized(&response),
                OutputFormat::Human => {
                    for resource in &response.results {
                        println!(
                            "{}\t{}\t{}",
                            resource.id,
                            resource.risk_tier,
                            resource.name.as_deref().unwrap_or("")
                        );
                    }
                    if response.results.is_empty() {
                        println!("no matching resources");
                    }
                }
            }
            ExitCode::SUCCESS
        }
        Err(error) => registry_error("search", &error, format),
    }
}

fn show_command(arguments: &[String], format: OutputFormat) -> ExitCode {
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        println!("USAGE:");
        println!("    ossus show <RESOURCE-ID> [--index PATH]");
        return ExitCode::SUCCESS;
    }
    let (id, index_path) = match parse_show_options(arguments) {
        Ok(parsed) => parsed,
        Err(message) => return usage_error("show", &message),
    };
    match show_resource(&index_path, &id) {
        Ok(response) => {
            if response.resource.is_none() {
                eprintln!(
                    "ossus show: resource `{}` was not found",
                    bounded_for_cli(&id)
                );
                return ExitCode::from(EXIT_REGISTRY);
            }
            match format {
                OutputFormat::Json => print_serialized(&response),
                OutputFormat::Human => {
                    if let Some(resource) = response.resource {
                        println!("id: {}", resource.id);
                        println!("name: {}", resource.name.as_deref().unwrap_or(""));
                        println!("type: {}", resource.resource_type);
                        println!(
                            "description: {}",
                            resource.description.as_deref().unwrap_or("")
                        );
                        println!(
                            "source: {}@{}",
                            resource.source_repository, resource.source_commit
                        );
                        println!("risk: {}", resource.risk_tier);
                        println!("review: {}", resource.review_status);
                        println!("capabilities: {}", resource.capabilities.join(", "));
                    }
                }
            }
            ExitCode::SUCCESS
        }
        Err(error) => registry_error("show", &error, format),
    }
}

fn parse_reindex_options(arguments: &[String]) -> Result<(PathBuf, PathBuf), String> {
    let mut manifest_root = PathBuf::from(DEFAULT_MANIFEST_ROOT);
    let mut index_path = PathBuf::from(DEFAULT_INDEX_PATH);
    let mut index = 0;
    while index < arguments.len() {
        let option = arguments[index].as_str();
        let Some(value) = arguments.get(index + 1) else {
            return Err(format!("{option} requires a path"));
        };
        match option {
            "--manifest-root" => manifest_root = PathBuf::from(value),
            "--index" => index_path = PathBuf::from(value),
            _ => return Err(format!("unexpected option `{}`", bounded_for_cli(option))),
        }
        index += 2;
    }
    Ok((manifest_root, index_path))
}

fn parse_single_path_option(
    arguments: &[String],
    option: &str,
    default: &str,
) -> Result<PathBuf, String> {
    if arguments.is_empty() {
        return Ok(PathBuf::from(default));
    }
    if arguments.len() != 2 || arguments[0] != option {
        return Err(format!("expected [{option} PATH]"));
    }
    Ok(PathBuf::from(&arguments[1]))
}

fn parse_show_options(arguments: &[String]) -> Result<(String, PathBuf), String> {
    let Some(id) = arguments.first() else {
        return Err(String::from("a resource ID is required"));
    };
    if id.starts_with('-') {
        return Err(String::from("a resource ID must precede options"));
    }
    let path = parse_single_path_option(&arguments[1..], "--index", DEFAULT_INDEX_PATH)?;
    Ok((id.clone(), path))
}

fn parse_search_options(arguments: &[String]) -> Result<(PathBuf, SearchQuery), String> {
    let mut path = PathBuf::from(DEFAULT_INDEX_PATH);
    let mut query = SearchQuery::default();
    let mut text = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        let argument = &arguments[index];
        if !argument.starts_with('-') {
            text.push(argument.clone());
            index += 1;
            continue;
        }
        let Some(value) = arguments.get(index + 1) else {
            return Err(format!("{argument} requires a value"));
        };
        match argument.as_str() {
            "--index" => path = PathBuf::from(value),
            "--exact" => query.exact = Some(value.clone()),
            "--capability" => query.capabilities.push(value.clone()),
            "--category" => query
                .categories
                .push(CategoryName::parse(value).map_err(|error| error.to_string())?),
            "--surface" => query
                .surfaces
                .push(Surface::try_from(value.as_str()).map_err(|error| error.to_string())?),
            "--source" | "--source-mode" => query
                .source_modes
                .push(SourceMode::try_from(value.as_str()).map_err(|error| error.to_string())?),
            "--runtime" => query.runtimes.push(
                RuntimeRequirement::try_from(value.as_str()).map_err(|error| error.to_string())?,
            ),
            "--risk-max" => {
                query.maximum_risk =
                    Some(RiskTier::try_from(value.as_str()).map_err(|error| error.to_string())?)
            }
            "--limit" => {
                query.limit = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| String::from("--limit requires an unsigned integer"))?,
                )
            }
            "--offset" => {
                query.offset = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| String::from("--offset requires an unsigned integer"))?,
                )
            }
            _ => return Err(format!("unexpected option `{}`", bounded_for_cli(argument))),
        }
        index += 2;
    }
    if !text.is_empty() {
        query.text = Some(text.join(" "));
    }
    Ok((path, query))
}

fn registry_error(context: &str, error: &RegistryError, format: OutputFormat) -> ExitCode {
    let conflicts = match error {
        RegistryError::Conflict(conflicts) => conflicts.as_slice(),
        _ => &[],
    };
    match format {
        OutputFormat::Json => print_serialized(&ErrorOutput {
            schema_version: ossus_registry::OUTPUT_SCHEMA_VERSION,
            error: "registry-error",
            message: error.to_string(),
            conflicts,
        }),
        OutputFormat::Human => {
            eprintln!("ossus {context}: {error}");
            for conflict in conflicts {
                eprintln!(
                    "  {} {}: {}",
                    conflict.reason_code,
                    conflict.identity,
                    conflict.paths.join(", ")
                );
            }
        }
    }
    ExitCode::from(EXIT_REGISTRY)
}

fn usage_error(context: &str, message: &str) -> ExitCode {
    eprintln!("ossus {context}: {message}");
    ExitCode::from(EXIT_USAGE)
}

fn print_serialized(value: &impl Serialize) {
    match serde_json::to_string(value) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("ossus: unable to serialize output: {error}");
            std::process::exit(i32::from(EXIT_INTERNAL));
        }
    }
}

struct ValidationRecord {
    display_path: String,
    diagnostics: Vec<Diagnostic>,
}

fn validate_command(arguments: &[String], format: OutputFormat) -> ExitCode {
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print_validate_help();
        return ExitCode::SUCCESS;
    }
    if arguments.is_empty() {
        eprintln!("ossus validate: at least one manifest PATH is required");
        eprintln!("next: run 'ossus validate <PATH>'");
        return ExitCode::from(EXIT_USAGE);
    }
    if arguments.iter().any(|argument| argument.starts_with('-')) {
        eprintln!("ossus validate: unexpected option; use --format human|json");
        return ExitCode::from(EXIT_USAGE);
    }

    let budget = Budget::default();
    let records = arguments
        .iter()
        .map(|argument| {
            let path = PathBuf::from(argument);
            let display_path = normalized_path(&path);
            let diagnostics = match load_canonical_manifest_report(&path, &budget) {
                Ok(report) => report.diagnostics,
                Err(diagnostics) => diagnostics,
            };
            ValidationRecord {
                display_path,
                diagnostics,
            }
        })
        .collect::<Vec<_>>();

    match format {
        OutputFormat::Human => print_human_validation(&records),
        OutputFormat::Json => print_json_validation(&records),
    }
    print_security_warnings(&records);

    let has_schema_error = records.iter().any(|record| {
        record.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.class == DiagnosticClass::Schema
        })
    });
    if has_schema_error {
        return ExitCode::from(EXIT_SCHEMA);
    }
    let has_taxonomy_error = records.iter().any(|record| {
        record.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.class == DiagnosticClass::Taxonomy
        })
    });
    if has_taxonomy_error {
        return ExitCode::from(EXIT_TAXONOMY);
    }
    ExitCode::SUCCESS
}

fn print_human_validation(records: &[ValidationRecord]) {
    for record in records {
        let has_error = record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        if has_error {
            eprintln!("{}: invalid", record.display_path);
            for diagnostic in &record.diagnostics {
                if diagnostic.severity == DiagnosticSeverity::Error {
                    eprintln!(
                        "  {} {}: {}",
                        diagnostic.field_path, diagnostic.reason_code, diagnostic.message
                    );
                }
            }
        } else if record.diagnostics.is_empty() {
            println!("{}: valid", record.display_path);
        } else {
            println!("{}: valid with warnings", record.display_path);
        }
    }
    if records.iter().any(|record| {
        record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    }) {
        eprintln!("next: correct the reported contract fields and rerun 'ossus validate <PATH>'");
    } else {
        println!("next: submit valid manifests for the appropriate Registry review");
    }
}

fn print_security_warnings(records: &[ValidationRecord]) {
    for record in records {
        for diagnostic in &record.diagnostics {
            if diagnostic.severity == DiagnosticSeverity::Warning {
                eprintln!(
                    "{}: warning {} {}: {}",
                    record.display_path,
                    diagnostic.field_path,
                    diagnostic.reason_code,
                    diagnostic.message
                );
            }
        }
    }
}

fn print_json_validation(records: &[ValidationRecord]) {
    let mut output = String::from("{\"schema_version\":\"1.0.0\",\"results\":[");
    for (record_index, record) in records.iter().enumerate() {
        if record_index > 0 {
            output.push(',');
        }
        let valid = !record
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        output.push_str("{\"path\":");
        output.push_str(&json_string(&record.display_path));
        output.push_str(",\"valid\":");
        output.push_str(if valid { "true" } else { "false" });
        output.push_str(",\"diagnostics\":[");
        for (diagnostic_index, diagnostic) in record.diagnostics.iter().enumerate() {
            if diagnostic_index > 0 {
                output.push(',');
            }
            output.push_str("{\"severity\":");
            output.push_str(&json_string(diagnostic.severity.as_str()));
            output.push_str(",\"class\":");
            output.push_str(&json_string(diagnostic.class.as_str()));
            output.push_str(",\"reason_code\":");
            output.push_str(&json_string(&diagnostic.reason_code));
            output.push_str(",\"field_path\":");
            output.push_str(&json_string(&diagnostic.field_path));
            output.push_str(",\"message\":");
            output.push_str(&json_string(&diagnostic.message));
            output.push('}');
        }
        output.push_str("]}");
    }
    output.push_str("]}\n");
    print!("{output}");
}

fn json_string(value: &str) -> String {
    let mut escaped = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                let code = character as u32;
                escaped.push_str("\\u00");
                escaped.push(hex_digit((code >> 4) & 0x0f));
                escaped.push(hex_digit(code & 0x0f));
            }
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

fn hex_digit(value: u32) -> char {
    match value {
        0..=9 => char::from(b'0' + value as u8),
        10..=15 => char::from(b'a' + (value as u8 - 10)),
        _ => '0',
    }
}

fn bounded_for_cli(value: &str) -> String {
    ossus_core::bounded_display_value(value)
}

fn normalized_path(path: &Path) -> String {
    // A manifest path can come from an imported catalog directory, so the file
    // name is untrusted even when the invocation is not. The path is escaped but
    // deliberately not truncated: the reader must still see which file failed.
    ossus_core::escaped_display_value(&raw_normalized_path(path))
}

fn raw_normalized_path(path: &Path) -> String {
    if let Ok(canonical) = path.canonicalize() {
        return canonical.display().to_string();
    }
    if path.is_absolute() {
        return path.display().to_string();
    }
    match env::current_dir() {
        Ok(current) => current.join(path).display().to_string(),
        Err(_) => path.display().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::process::ExitCode;

    #[test]
    fn help_succeeds() {
        assert_eq!(run(["help".to_owned()].into_iter()), ExitCode::SUCCESS);
    }

    #[test]
    fn unknown_command_fails_with_usage_code() {
        assert_eq!(run(["unknown".to_owned()].into_iter()), ExitCode::from(2));
    }

    #[test]
    fn future_command_is_explicitly_unavailable() {
        assert_eq!(run(["resolve".to_owned()].into_iter()), ExitCode::from(69));
    }
}
