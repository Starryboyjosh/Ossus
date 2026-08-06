#![forbid(unsafe_code)]

use std::{
    env,
    path::{Path, PathBuf},
    process::ExitCode,
};

use ossus_core::{CURRENT_WAVE, DOMAINS, PRODUCT_NAME};
use ossus_registry::{
    Budget, Diagnostic, DiagnosticClass, DiagnosticSeverity, load_canonical_manifest_report,
};

const EXIT_USAGE: u8 = 2;
const EXIT_SCHEMA: u8 = 11;
const EXIT_TAXONOMY: u8 = 12;
const EXIT_NOT_IMPLEMENTED: u8 = 69;

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
        "init" | "config" | "registry" | "search" | "show" | "scan" | "resolve" | "explain"
        | "activate" | "deactivate" | "lock" | "doctor" | "eval" | "audit" | "research" => {
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
    println!("VALIDATION COMMANDS:");
    println!("    validate   Validate canonical manifest files");
    println!();
    println!("PLANNED COMMAND GROUPS:");
    println!("    init config registry search show scan resolve explain");
    println!("    activate deactivate lock doctor eval audit research");
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
    println!("repository_state: scaffold");
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
