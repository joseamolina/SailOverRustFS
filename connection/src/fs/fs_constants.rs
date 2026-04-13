// ── Size ─────────────────────────────────────────────────────────────────────

/// Raw byte count of a file entry.
pub const SIZE: &str = "size";
/// Human-readable size string (e.g. "4.2 MB").
pub const HUMAN_READABLE_SIZE: &str = "hrSize";

// ── Path / type ───────────────────────────────────────────────────────────────

/// Absolute or storage-relative path of the entry.
pub const PATH: &str = "path";
/// `"true"` when the entry is a directory or S3 common-prefix.
pub const IS_DIRECTORY: &str = "isDirectory";
/// `"true"` for entries that are neither regular files nor directories (e.g. device nodes).
pub const IS_OTHER: &str = "isOther";
/// `"true"` when the entry is a regular file.
pub const IS_REGULAR_FILE: &str = "isRegularFile";
/// `"true"` when the entry is a symbolic link.
pub const IS_SYMBOLIC_LINK: &str = "isSymbolicLink";

// ── Partitioning ──────────────────────────────────────────────────────────────

/// JSON array of file paths that belong to a single logical partition.
pub const PARTITION_FILES: &str = "partitionFiles";
/// Total number of partitions the dataset is split into.
pub const PARTITIONS: &str = "partitions";
/// Storage or serialisation format of the entry (e.g. `"parquet"`, `"csv"`).
pub const FORMAT: &str = "format";

// ── Timestamps ────────────────────────────────────────────────────────────────

/// Creation time as milliseconds since the Unix epoch.
pub const CREATION_TIME: &str = "creationTime";
/// Human-readable creation time string.
pub const HUMAN_READABLE_CREATION_TIME: &str = "hrCreationTime";
/// Last-modified time as milliseconds since the Unix epoch.
pub const LAST_MODIFIED: &str = "lastModified";
/// Human-readable last-modified time string.
pub const HUMAN_READABLE_LAST_MODIFIED: &str = "hrLastModified";
