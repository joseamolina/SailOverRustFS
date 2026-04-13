/// Filesystem traversal and resource extraction over an S3-backed [`Operator`].
pub mod fs_extractor;

/// Core trait describing a generic filesystem entry (file or directory).
pub mod fs_file;

/// String keys used when serialising file metadata into attribute maps.
pub mod fs_constants;

/// Pluggable metadata extraction engines keyed by file/folder type.
pub mod engine;

/// Internal helpers for wiring an [`Operator`] into the filesystem layer.
mod fs_connect;