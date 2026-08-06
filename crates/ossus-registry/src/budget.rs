//! Bounded parsing primitives for trusted Registry input.

use std::str;

use toml::Value;

use crate::diagnostic::{Diagnostic, DiagnosticClass, bounded_value};

/// Limits applied while parsing a trusted contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Budget {
    pub manifest_bytes: usize,
    pub task_bytes: usize,
    pub config_bytes: usize,
    pub max_string_length: usize,
    pub max_list_items: usize,
    pub max_nesting_depth: usize,
    pub max_manifests_per_source: usize,
    pub max_project_files: usize,
    pub max_project_bytes: usize,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            manifest_bytes: 262_144,
            task_bytes: 65_536,
            config_bytes: 262_144,
            max_string_length: 32_768,
            max_list_items: 2_000,
            max_nesting_depth: 32,
            max_manifests_per_source: 50_000,
            max_project_files: 100_000,
            max_project_bytes: 67_108_864,
        }
    }
}

impl Budget {
    /// Restricts every field to the lower of the two budgets.
    #[must_use]
    pub fn restrict(self, other: &Self) -> Self {
        Self {
            manifest_bytes: self.manifest_bytes.min(other.manifest_bytes),
            task_bytes: self.task_bytes.min(other.task_bytes),
            config_bytes: self.config_bytes.min(other.config_bytes),
            max_string_length: self.max_string_length.min(other.max_string_length),
            max_list_items: self.max_list_items.min(other.max_list_items),
            max_nesting_depth: self.max_nesting_depth.min(other.max_nesting_depth),
            max_manifests_per_source: self
                .max_manifests_per_source
                .min(other.max_manifests_per_source),
            max_project_files: self.max_project_files.min(other.max_project_files),
            max_project_bytes: self.max_project_bytes.min(other.max_project_bytes),
        }
    }
}

/// Parses a UTF-8 TOML source after applying size, depth, string, and list budgets.
pub(crate) fn parse_bounded_toml(
    source: &str,
    budget: &Budget,
    class: DiagnosticClass,
) -> Result<Value, Vec<Diagnostic>> {
    if source.len() > budget.manifest_bytes {
        return Err(vec![Diagnostic::error(
            class,
            "budget.manifest-bytes.exceeded",
            "$",
            "TOML input exceeds the configured manifest byte budget",
        )]);
    }

    if preparse_nesting_depth(source.as_bytes(), budget.max_nesting_depth).is_some() {
        return Err(vec![Diagnostic::error(
            class,
            "budget.nesting-depth.exceeded",
            "$",
            format!(
                "TOML nesting depth exceeds the configured limit of {}",
                budget.max_nesting_depth
            ),
        )]);
    }

    let value = match toml::from_str::<Value>(source) {
        Ok(value) => value,
        Err(_) => {
            return Err(vec![Diagnostic::error(
                class,
                "manifest.toml.invalid",
                "$",
                "input is not valid TOML",
            )]);
        }
    };

    let mut diagnostics = Vec::new();
    walk_value(&value, budget, class, &mut diagnostics);
    if diagnostics.is_empty() {
        Ok(value)
    } else {
        crate::diagnostic::sort_diagnostics(&mut diagnostics);
        Err(diagnostics)
    }
}

fn walk_value(
    root: &Value,
    budget: &Budget,
    class: DiagnosticClass,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut pending = vec![(root, 0_usize, String::from("$"))];

    while let Some((value, depth, path)) = pending.pop() {
        match value {
            Value::String(value) => {
                // `DATA_CONTRACTS.md` states this budget in KiB, which is a byte
                // measure, and the budget exists to bound memory. Counting chars
                // instead admitted 4 bytes per unit, so a 32_768-unit string of
                // astral-plane characters occupied 128 KiB against a 32 KiB limit.
                // Schema field limits are a different rule and stay in code points,
                // matching JSON Schema `maxLength`; see `validate::check_string`.
                if value.len() > budget.max_string_length {
                    diagnostics.push(Diagnostic::error(
                        class,
                        "budget.string-length.exceeded",
                        path,
                        "a string exceeds the configured string-length budget",
                    ));
                }
            }
            Value::Array(items) => {
                if depth > budget.max_nesting_depth {
                    diagnostics.push(Diagnostic::error(
                        class,
                        "budget.nesting-depth.exceeded",
                        path.clone(),
                        "a TOML value exceeds the configured nesting-depth budget",
                    ));
                    continue;
                }
                if items.len() > budget.max_list_items {
                    diagnostics.push(Diagnostic::error(
                        class,
                        "budget.list-items.exceeded",
                        path.clone(),
                        "a list exceeds the configured item-count budget",
                    ));
                }
                for (index, item) in items.iter().enumerate().rev() {
                    pending.push((item, depth.saturating_add(1), format!("{path}[{index}]")));
                }
            }
            Value::Table(table) => {
                if depth > budget.max_nesting_depth {
                    diagnostics.push(Diagnostic::error(
                        class,
                        "budget.nesting-depth.exceeded",
                        path,
                        "a TOML value exceeds the configured nesting-depth budget",
                    ));
                    continue;
                }
                for (key, item) in table.iter().rev() {
                    // The key is attacker-controlled and this path is rendered
                    // into a diagnostic, so each segment is bounded and escaped.
                    // Building the path from the raw key left it both unbounded
                    // in length and able to carry terminal escape sequences.
                    let key = bounded_value(key);
                    pending.push((
                        item,
                        depth.saturating_add(1),
                        if path == "$" {
                            key
                        } else {
                            format!("{path}.{key}")
                        },
                    ));
                }
            }
            Value::Integer(_) | Value::Float(_) | Value::Boolean(_) | Value::Datetime(_) => {}
        }
    }
}

/// Performs a cheap lexical nesting scan before invoking the TOML parser.
///
/// The parser is deliberately not trusted to be the first depth gate: a deeply
/// nested hostile input must produce a bounded diagnostic even if a parser
/// implementation would recurse before returning an error.
fn preparse_nesting_depth(bytes: &[u8], maximum: usize) -> Option<usize> {
    #[derive(Clone, Copy)]
    enum Mode {
        Normal,
        Comment,
        Basic,
        Literal,
        BasicMultiline,
        LiteralMultiline,
    }

    let mut mode = Mode::Normal;
    let mut depth = 0_usize;
    let mut index = 0_usize;

    while index < bytes.len() {
        let byte = bytes[index];
        match mode {
            Mode::Normal => match byte {
                b'#' => mode = Mode::Comment,
                b'"' => {
                    if bytes.get(index + 1) == Some(&b'"') && bytes.get(index + 2) == Some(&b'"') {
                        mode = Mode::BasicMultiline;
                        index = index.saturating_add(2);
                    } else {
                        mode = Mode::Basic;
                    }
                }
                b'\'' => {
                    if bytes.get(index + 1) == Some(&b'\'') && bytes.get(index + 2) == Some(&b'\'')
                    {
                        mode = Mode::LiteralMultiline;
                        index = index.saturating_add(2);
                    } else {
                        mode = Mode::Literal;
                    }
                }
                b'{' | b'[' => {
                    depth = depth.saturating_add(1);
                    if depth > maximum {
                        return Some(depth);
                    }
                }
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => {}
            },
            Mode::Comment => {
                if byte == b'\n' || byte == b'\r' {
                    mode = Mode::Normal;
                }
            }
            Mode::Basic => {
                if byte == b'\\' {
                    index = index.saturating_add(1);
                } else if byte == b'"' {
                    mode = Mode::Normal;
                }
            }
            Mode::Literal => {
                if byte == b'\'' {
                    mode = Mode::Normal;
                }
            }
            Mode::BasicMultiline => {
                // Multi-line basic strings process escapes, so a `\"` is content
                // and must not be mistaken for the start of a closing delimiter.
                if byte == b'\\' {
                    index = index.saturating_add(1);
                } else if byte == b'"' {
                    let run = delimiter_run(bytes, index, b'"');
                    if run >= 3 {
                        mode = Mode::Normal;
                    }
                    index = index.saturating_add(run.saturating_sub(1));
                }
            }
            Mode::LiteralMultiline => {
                // Literal strings have no escapes, so only the quote run matters.
                if byte == b'\'' {
                    let run = delimiter_run(bytes, index, b'\'');
                    if run >= 3 {
                        mode = Mode::Normal;
                    }
                    index = index.saturating_add(run.saturating_sub(1));
                }
            }
        }
        index = index.saturating_add(1);
    }

    None
}

/// Length of the run of `delimiter` bytes starting at `start`.
///
/// A multi-line string may legally end with up to two content quotes directly
/// against its closing delimiter, so `"""a""""` is the string `a"`. Consuming a
/// fixed three bytes on close left the trailing quote to be read as the start of
/// a new string, which silently disabled this scan for the rest of the file.
fn delimiter_run(bytes: &[u8], start: usize, delimiter: u8) -> usize {
    bytes[start..]
        .iter()
        .take_while(|byte| **byte == delimiter)
        .count()
}

/// Returns a UTF-8 diagnostic without exposing the offending byte sequence.
pub(crate) fn utf8_diagnostic(class: DiagnosticClass) -> Diagnostic {
    Diagnostic::error(
        class,
        "encoding.utf8.invalid",
        "$",
        "input is not valid UTF-8",
    )
}

/// Converts bytes to UTF-8 while keeping invalid input in the diagnostic layer.
pub(crate) fn decode_utf8(bytes: Vec<u8>, class: DiagnosticClass) -> Result<String, Diagnostic> {
    match str::from_utf8(&bytes) {
        Ok(source) => Ok(source.to_owned()),
        Err(_) => Err(utf8_diagnostic(class)),
    }
}
