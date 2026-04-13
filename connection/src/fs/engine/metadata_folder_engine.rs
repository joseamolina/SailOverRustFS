use crate::fs::engine::metadata_engine::MetadataEngine;
use crate::fs::fs_file::FsFile;

/// Specialisation of [`MetadataEngine`] for engines that process directories.
pub trait MetadataFolderEngine: MetadataEngine {
    /// Returns `true` when this engine knows how to handle the given `folder`.
    ///
    /// Implementations typically inspect the folder's name, structure, or contents
    /// (e.g. detecting a Hive-partitioned layout or a Delta Lake `_delta_log` directory).
    fn can_handle(folder: &dyn FsFile) -> bool;
}
