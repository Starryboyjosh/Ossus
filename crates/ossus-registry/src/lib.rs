#![forbid(unsafe_code)]

//! Trusted canonical metadata and local index.
//!
//! Canonical manifest loading, bounded parsing, taxonomy lookup, semantic
//! validation, and the disposable local search index live at this trusted
//! boundary.

use ossus_core::ComponentState;

mod budget;
mod diagnostic;
mod index;
mod manifest;
mod taxonomy;
mod validate;

pub use budget::Budget;
pub use diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity};
pub use index::{
    ExcludedManifest, INDEX_SCHEMA_VERSION, OUTPUT_SCHEMA_VERSION, RegistryConflict, RegistryError,
    RegistryStatus, ReindexReport, ResourceRecord, SearchQuery, SearchResults, ShowResult,
    rebuild_index, registry_status, search_index, show_resource,
};
pub use manifest::{
    CanonicalManifest, Capabilities, Compatibility, Context, Distribution, ManifestValidation,
    Review, Risk, Runtime, Source, load_canonical_manifest, load_canonical_manifest_report,
    parse_canonical_manifest, parse_canonical_manifest_report,
    parse_canonical_manifest_with_taxonomy,
};
pub use taxonomy::{CapabilityDefinition, DeprecationPolicy, Taxonomy};
pub use validate::validate_manifest;

/// Display label used by the CLI and component inventory.
pub const COMPONENT_NAME: &str = "Registry";

/// Reports that canonical manifest and taxonomy validation is implemented.
#[must_use]
pub const fn component_state() -> ComponentState {
    ComponentState::Implemented
}

#[cfg(test)]
mod tests {
    use super::component_state;
    use ossus_core::ComponentState;

    #[test]
    fn reports_implemented_contract_loading() {
        assert_eq!(component_state(), ComponentState::Implemented);
    }
}
