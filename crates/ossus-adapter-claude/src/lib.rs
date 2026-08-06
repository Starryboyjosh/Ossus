#![forbid(unsafe_code)]

//! Host materialization boundary for Claude Code.
//!
//! Product behavior is intentionally deferred to the assigned implementation WAVE.

use ossus_core::ComponentState;

/// Display label used by the bootstrap CLI.
pub const COMPONENT_NAME: &str = "Claude Code adapter";

/// Reports that only repository structure exists.
#[must_use]
pub const fn component_state() -> ComponentState {
    ComponentState::Scaffolded
}

#[cfg(test)]
mod tests {
    use super::component_state;
    use ossus_core::ComponentState;

    #[test]
    fn remains_a_scaffold_until_its_wave_is_implemented() {
        assert_eq!(component_state(), ComponentState::Scaffolded);
    }
}
