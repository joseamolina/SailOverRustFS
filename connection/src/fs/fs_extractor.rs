use rayon::prelude::*;
use log::{info, debug};
use std::error::Error;
use walkdir::WalkDir;
use opendal::{ErrorKind, Operator};

/// A lightweight descriptor for a file or object that has been extracted and catalogued.
#[derive(Debug, Clone)]
pub struct CustomResource {
    pub path: String,
}

/// Default metadata extraction engine used when no specialised engine matches a file.
#[derive(Clone)]
pub struct Engine;

impl Engine {
    /// Converts a [`File`] into a [`CustomResource`] by reading its schema.
    ///
    /// Currently a placeholder: returns a resource whose path mirrors the source file.
    /// Replace with real schema-extraction logic (e.g. reading Parquet/Avro footers).
    pub fn get_schema_as_custom_resource(&self, file: &File) -> Result<CustomResource, Box<dyn Error + Send + Sync>> {
        Ok(CustomResource {
            path: file.full_path.clone(),
        })
    }
}

/// A resolved filesystem entry with its absolute storage path.
#[derive(Debug, Clone)]
pub struct File {
    pub full_path: String,
}

/// Entry point for traversing a path and extracting [`CustomResource`] metadata.
pub struct FsExtractor;

impl FsExtractor {

    /// Checks whether `path` exists in the storage backend.
    ///
    /// Returns `Ok(true)` if the object/prefix is found, `Ok(false)` if it is absent,
    /// and propagates any other OpenDAL error as `Err`.
    pub async fn check_exists(path: &str, op: Operator) -> opendal::Result<bool> {
        match op.stat(path).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Lists all regular files (non-prefixes) directly under `path` in the storage backend.
    ///
    /// Entries that cannot be stat-ed are silently skipped.
    pub async fn files(path: &str, op: Operator) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = op.list(path).await {
            for entry in entries {
                if let Ok(meta) = op.stat(entry.path()).await {
                    if meta.is_file() {
                        files.push(entry.path().to_string());
                    }
                }
            }
        }
        files
    }

    /// Recursively walks `path`, extracts metadata from each file via its associated engine,
    /// and returns both the full list of discovered file paths and the successfully extracted
    /// [`CustomResource`] descriptors.
    ///
    /// Files that fail extraction (or have no matching engine) are excluded from the resource
    /// list but still appear in the returned path list.
    ///
    /// Extraction runs in parallel via Rayon for better throughput on large directory trees.
    pub fn extract(path: &str) -> (Vec<String>, Vec<CustomResource>) {
        info!("Starting extraction for path: {path}");

        let all_files = Self::files_with_metadata_engines(path);

        // Pair each file with its engine, then run extraction in parallel.
        let resources: Vec<CustomResource> = all_files
            .par_iter()
            .filter_map(|(file, maybe_engine)| {
                if let Some(engine) = maybe_engine {
                    match engine.get_schema_as_custom_resource(file) {
                        Ok(custom_resource) => {
                            debug!("Extracted custom resource from {}", file.full_path);
                            Some(custom_resource)
                        }
                        Err(e) => {
                            debug!("Failed extracting resource from {}: {:?}", file.full_path, e);
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

        // Separate the path list from the engine pairings before returning.
        let file_paths = all_files.into_iter().map(|(f, _)| f.full_path).collect();

        (file_paths, resources)
    }

    /// Walks `path` recursively and pairs each regular file with the [`Engine`] that should
    /// process it.  Returns `None` for the engine slot when no suitable engine is found.
    fn files_with_metadata_engines(path: &str) -> Vec<(File, Option<Engine>)> {
        let mut results = Vec::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file = File {
                    full_path: entry.path().to_string_lossy().to_string(),
                };
                // TODO: replace with real engine-selection logic keyed on file extension / format.
                results.push((file, Some(Engine)));
            }
        }
        results
    }
}
