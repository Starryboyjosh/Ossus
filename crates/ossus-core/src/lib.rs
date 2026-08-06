#![forbid(unsafe_code)]

//! Shared domain contracts for Ossus.
//!
//! The crate is intentionally small during the repository scaffold. Product
//! types are added only by their assigned implementation WAVEs.

use std::fmt;

mod display;
mod enums;
mod error;
mod ids;
mod version;

pub use display::{bounded_display_value, escaped_display_value, is_display_unsafe};
pub use enums::{
    CapabilityStatus, DistributionMode, Portability, ResourceType, ReviewStatus, ReviewTier,
    RiskTier, RuntimeRequirement, Scope, SourceMode, Surface, distribution_is_permitted,
};
pub use error::ContractError;
pub use ids::{CapabilityId, CategoryName, CommitHash, ResourceId, TreeHash};
pub use version::{SCHEMA_VERSION, Version, validate_schema_version};

/// Human-readable product name.
pub const PRODUCT_NAME: &str = "Ossus";

/// Current implementation WAVE for the repository.
pub const CURRENT_WAVE: &str = "WAVE-002";

/// The three permanent product domains.
pub const DOMAINS: [&str; 3] = ["Researcher", "Registry", "Resolver"];

/// Coarse component state used by the bootstrap status command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentState {
    /// The component exists only in approved planning documents.
    Planned,
    /// The component has repository structure but no product behavior.
    Scaffolded,
    /// The component has implemented product behavior.
    Implemented,
}

impl fmt::Display for ComponentState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Planned => "planned",
            Self::Scaffolded => "scaffolded",
            Self::Implemented => "implemented",
        };
        formatter.write_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{CURRENT_WAVE, DOMAINS, PRODUCT_NAME};

    #[test]
    fn exposes_stable_bootstrap_metadata() {
        assert_eq!(PRODUCT_NAME, "Ossus");
        assert_eq!(CURRENT_WAVE, "WAVE-002");
        assert_eq!(DOMAINS, ["Researcher", "Registry", "Resolver"]);
    }
}
