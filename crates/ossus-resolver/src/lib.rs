#![forbid(unsafe_code)]

//! Deterministic Resolver domain.
//!
//! Selection behavior is implemented in WAVE-005 after the Registry and project
//! scanner contracts exist.

use ossus_core::ComponentState;

/// Display label used by the bootstrap CLI.
pub const COMPONENT_NAME: &str = "Resolver";

/// Reports that only repository structure exists.
#[must_use]
pub const fn component_state() -> ComponentState {
    ComponentState::Scaffolded
}

/// The planned default active-resource limit.
pub const DEFAULT_ACTIVE_RESOURCE_LIMIT: usize = 5;

#[cfg(test)]
mod tests {
    use super::{DEFAULT_ACTIVE_RESOURCE_LIMIT, component_state};
    use ossus_core::ComponentState;

    #[test]
    fn exposes_only_bootstrap_constants() {
        assert_eq!(component_state(), ComponentState::Scaffolded);
        assert_eq!(DEFAULT_ACTIVE_RESOURCE_LIMIT, 5);
    }
}
