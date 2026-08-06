//! Deterministic diagnostics emitted while loading trusted Registry contracts.

use std::cmp::Ordering;

use ossus_core::ContractError;

/// The contract boundary at which a diagnostic was produced.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticClass {
    /// The manifest or taxonomy has invalid shape, syntax, or contract data.
    Schema,
    /// The manifest cannot be admitted because governed taxonomy state disagrees.
    Taxonomy,
}

impl DiagnosticClass {
    /// Returns the stable wire value used by machine-readable CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Taxonomy => "taxonomy",
        }
    }
}

/// The severity of a diagnostic.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticSeverity {
    /// The input cannot be admitted.
    Error,
    /// The input remains valid, but the condition must remain visible.
    Warning,
}

impl DiagnosticSeverity {
    /// Returns the stable wire value used by machine-readable CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// A bounded, stable diagnostic for a malformed or semantically invalid input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Whether the diagnostic is a schema or taxonomy failure.
    pub class: DiagnosticClass,
    /// Whether the condition is fatal or a visible warning.
    pub severity: DiagnosticSeverity,
    /// Stable machine-readable reason code.
    pub reason_code: String,
    /// Dotted path into the input contract.
    pub field_path: String,
    /// Human-readable explanation, with hostile values bounded.
    pub message: String,
}

impl Diagnostic {
    /// Constructs an error diagnostic.
    pub fn error(
        class: DiagnosticClass,
        reason_code: impl Into<String>,
        field_path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            class,
            severity: DiagnosticSeverity::Error,
            reason_code: reason_code.into(),
            field_path: field_path.into(),
            message: message.into(),
        }
    }

    /// Constructs a warning diagnostic.
    pub fn warning(
        class: DiagnosticClass,
        reason_code: impl Into<String>,
        field_path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            class,
            severity: DiagnosticSeverity::Warning,
            reason_code: reason_code.into(),
            field_path: field_path.into(),
            message: message.into(),
        }
    }

    /// Constructs a diagnostic from an existing core contract error.
    pub fn from_contract_error(
        class: DiagnosticClass,
        field_path: impl Into<String>,
        error: &ContractError,
    ) -> Self {
        Self::error(class, error.reason_code(), field_path, error.to_string())
    }

    /// Returns true when this diagnostic prevents admission.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, DiagnosticSeverity::Error)
    }

    /// Returns the stable reason code without exposing the backing representation.
    pub fn reason_code(&self) -> &str {
        &self.reason_code
    }

    /// Returns the dotted field path associated with this diagnostic.
    pub fn field_path(&self) -> &str {
        &self.field_path
    }
}

/// Bounds a value for inclusion in a diagnostic without exposing candidate content.
///
/// Delegates to the single shared rule in `ossus_core` so that length bounding
/// and terminal-escape neutralization cannot drift apart between the crates that
/// render untrusted values.
pub fn bounded_value(value: &str) -> String {
    ossus_core::bounded_display_value(value)
}

/// Sorts diagnostics by field path and then by stable metadata.
pub fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        left.field_path
            .cmp(&right.field_path)
            .then_with(|| left.reason_code.cmp(&right.reason_code))
            .then_with(|| left.severity.cmp(&right.severity))
            .then_with(|| left.message.cmp(&right.message))
            .then(Ordering::Equal)
    });
}
