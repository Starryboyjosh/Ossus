//! Stable errors exposed by trusted contract parsing.

use std::fmt;

/// A validation error for an Ossus trusted contract value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    InvalidResourceId { value: String },
    InvalidCapabilityId { value: String },
    InvalidCommitHash { value: String },
    InvalidTreeHash { value: String },
    InvalidCategoryName { value: String },
    InvalidVersion { value: String },
    UnsupportedVersionMajor { value: String },
    UnknownResourceType { value: String },
    UnknownSourceMode { value: String },
    UnknownDistributionMode { value: String },
    UnknownPortability { value: String },
    UnknownScope { value: String },
    UnknownRuntimeRequirement { value: String },
    UnknownRiskTier { value: String },
    UnknownReviewStatus { value: String },
    UnknownReviewTier { value: String },
    UnknownSurface { value: String },
    UnknownCapabilityStatus { value: String },
    DistributionModeContradictsSourceMode { value: String },
}

impl ContractError {
    /// Returns the stable, machine-observable reason code for this error.
    pub const fn reason_code(&self) -> &'static str {
        match self {
            Self::InvalidResourceId { .. } => "resource.id.invalid-format",
            Self::InvalidCapabilityId { .. } => "capability.id.invalid-format",
            Self::InvalidCommitHash { .. } => "source.commit.invalid-format",
            Self::InvalidTreeHash { .. } => "source.tree-hash.invalid-format",
            Self::InvalidCategoryName { .. } => "category.name.invalid-format",
            Self::InvalidVersion { .. } => "version.invalid-format",
            Self::UnsupportedVersionMajor { .. } => "version.unsupported-major",
            Self::UnknownResourceType { .. } => "resource-type.unknown",
            Self::UnknownSourceMode { .. } => "source.mode.unknown",
            Self::UnknownDistributionMode { .. } => "distribution.mode.unknown",
            Self::UnknownPortability { .. } => "portability.unknown",
            Self::UnknownScope { .. } => "scope.unknown",
            Self::UnknownRuntimeRequirement { .. } => "runtime.requirement.unknown",
            Self::UnknownRiskTier { .. } => "risk.tier.unknown",
            Self::UnknownReviewStatus { .. } => "review.status.unknown",
            Self::UnknownReviewTier { .. } => "review.tier.unknown",
            Self::UnknownSurface { .. } => "compatibility.surfaces.unknown",
            Self::UnknownCapabilityStatus { .. } => "capability.status.unknown",
            Self::DistributionModeContradictsSourceMode { .. } => {
                "distribution.mode.contradicts-source-mode"
            }
        }
    }

    fn value(&self) -> &str {
        match self {
            Self::InvalidResourceId { value }
            | Self::InvalidCapabilityId { value }
            | Self::InvalidCommitHash { value }
            | Self::InvalidTreeHash { value }
            | Self::InvalidCategoryName { value }
            | Self::InvalidVersion { value }
            | Self::UnsupportedVersionMajor { value }
            | Self::UnknownResourceType { value }
            | Self::UnknownSourceMode { value }
            | Self::UnknownDistributionMode { value }
            | Self::UnknownPortability { value }
            | Self::UnknownScope { value }
            | Self::UnknownRuntimeRequirement { value }
            | Self::UnknownRiskTier { value }
            | Self::UnknownReviewStatus { value }
            | Self::UnknownReviewTier { value }
            | Self::UnknownSurface { value }
            | Self::UnknownCapabilityStatus { value }
            | Self::DistributionModeContradictsSourceMode { value } => value,
        }
    }
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The rejected value is attacker-controlled and this string is printed
        // to a terminal, so it is bounded *and* escaped by the shared rule in
        // `crate::display` rather than written through verbatim.
        write!(
            formatter,
            "{}: `{}`",
            self.reason_code(),
            crate::display::bounded_display_value(self.value())
        )
    }
}

impl std::error::Error for ContractError {}

#[cfg(test)]
mod tests {
    use super::ContractError;

    #[test]
    fn display_truncates_hostile_values() {
        let error = ContractError::InvalidResourceId {
            value: "x".repeat(10_000),
        };
        let rendered = error.to_string();

        assert_eq!(
            rendered,
            format!("resource.id.invalid-format: `{}…`", "x".repeat(64))
        );
    }

    /// Truncation alone is not enough: a short value can still repaint a terminal.
    #[test]
    fn display_renders_terminal_control_sequences_inert() {
        let error = ContractError::UnknownRiskTier {
            value: String::from("R0\u{1b}[2J\napproved"),
        };
        let rendered = error.to_string();

        assert_eq!(rendered, "risk.tier.unknown: `R0\\u{1b}[2J\\u{a}approved`");
        assert!(!rendered.contains('\u{1b}'));
        assert!(!rendered.contains('\n'));
    }
}
