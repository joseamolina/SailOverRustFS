use crate::fs::fs_extractor::{FsExtractor, CustomResource};
use crate::fs::fs_file::FsFile;

/// Common interface for all metadata extraction engines.
///
/// Each engine is responsible for turning an [`FsFile`] into a [`CustomResource`] that
/// downstream consumers (e.g. a catalog or schema registry) can ingest.
pub trait MetadataEngine {
    /// Returns the shared [`FsExtractor`] instance used by this engine to traverse the storage.
    fn extractor() -> &'static FsExtractor;

    /// Converts a filesystem entry into a [`CustomResource`] containing its extracted metadata.
    fn get_schema_as_custom_resource(file: &dyn FsFile) -> CustomResource;

}
