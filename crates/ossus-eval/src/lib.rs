#![forbid(unsafe_code)]

//! Golden evaluation domain.
//!
//! The evaluation harness is implemented in WAVE-006. The frozen case data is
//! already present under the repository `evaluations/` directory.

use ossus_core::ComponentState;

/// Display label used by the bootstrap CLI.
pub const COMPONENT_NAME: &str = "Evaluation harness";

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
    fn remains_a_scaffold_until_wave_six() {
        assert_eq!(component_state(), ComponentState::Scaffolded);
    }
}
