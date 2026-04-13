/// Core trait shared by all metadata engines.
pub mod metadata_engine;

/// Extension of [`metadata_engine`] for engines that handle individual files.
mod metadata_file_engine;

/// Extension of [`metadata_engine`] for engines that handle directory-level metadata.
mod metadata_folder_engine;
