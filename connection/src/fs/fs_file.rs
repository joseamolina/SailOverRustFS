/// Abstraction over a single filesystem entry, whether it lives on a local disk or in S3.
///
/// Implementors provide uniform access to path components, size, timestamps, and any
/// storage-specific metadata through [`get_extra_attributes`].
pub trait FsFile {
    /// The bare filename or object key without any leading path segments.
    fn get_name(&self) -> String;

    /// The directory portion of the entry's path (i.e. everything except the final name segment).
    fn get_path(&self) -> String;

    /// The complete path including name, as understood by the backing storage.
    fn get_full_path(&self) -> String;

    /// The path of the parent directory, without a trailing separator.
    fn get_parent_path(&self) -> String;

    /// The full path of the parent directory, including any storage-specific prefix.
    fn get_parent_full_path(&self) -> String;

    /// Returns `true` when the entry is a directory (or a common-prefix in S3 terms).
    fn is_directory(&self) -> bool;

    /// Returns `true` when the entry is a regular file (or an S3 object with content).
    fn is_file(&self) -> bool;

    /// Size of the entry in bytes. Returns `0` for directories.
    fn get_length(&self) -> u64;

    /// Last-modified timestamp as milliseconds since the Unix epoch, if available.
    fn get_modification_time(&self) -> Option<u64>;

    /// Creation timestamp as milliseconds since the Unix epoch, if available.
    fn get_creation_time(&self) -> Option<u64>;

    /// Arbitrary key/value pairs carrying storage-specific metadata (e.g. ETag, content-type).
    fn get_extra_attributes(&self) -> std::collections::HashMap<String, String>;
}
