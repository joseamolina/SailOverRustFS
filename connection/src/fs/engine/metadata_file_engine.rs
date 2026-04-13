use crate::fs::engine::metadata_engine::MetadataEngine;

/// Specialisation of [`MetadataEngine`] for engines that process individual files.
pub trait MetadataFileEngine: MetadataEngine {
    /// Returns `true` when this engine is capable of processing files in the current context.
    ///
    /// Implementations typically inspect the runtime environment or configuration to decide
    /// whether they should be active (e.g. checking for a required dependency or feature flag).
    fn can_handle() -> bool;
}
