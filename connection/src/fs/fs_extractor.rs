use rayon::prelude::*;
use log::{info, debug};
use std::error::Error;
use walkdir::WalkDir;
use opendal::{ErrorKind, Operator};

#[derive(Debug, Clone)]
pub struct CustomResource {
    pub path: String,
}

#[derive(Clone)]
pub struct Engine;

impl Engine {
    pub fn get_schema_as_custom_resource(&self, file: &File) -> Result<CustomResource, Box<dyn Error + Send + Sync>> {
        // Placeholder logic: In a real scenario, this would read the file and extract schema
        Ok(CustomResource {
            path: file.full_path.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub full_path: String,
}

pub struct FsExtractor;

impl FsExtractor {

    pub async fn check_exists(path: &str, op: Operator) -> opendal::Result<bool> {
        match op.stat(path).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

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

    pub fn extract(path: &str) -> (Vec<String>, Vec<CustomResource>) {
        info!("Starting extraction for path: {path}");

        let all_files = Self::files_with_metadata_engines(path);

        // Collect all files within the specified path, along with their associated metadata and potential processing engines.
        // This collection will then be iterated over in parallel using Rayon for efficient extraction of custom resources.
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
        // Extract the file paths from the processed files, discarding the associated engine information.

        let file_paths = all_files.into_iter().map(|(f, _)| f.full_path).collect();

        (file_paths, resources)
    }

    fn files_with_metadata_engines(path: &str) -> Vec<(File, Option<Engine>)> {
        let mut results = Vec::new();
        // Use WalkDir to recursively find files
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file = File {
                    full_path: entry.path().to_string_lossy().to_string(),
                };
                // Logic to determine if it has an engine. 
                // For now, we'll assume all files have an engine for demonstration.
                results.push((file, Some(Engine)));
            }
        }
        results
    }
}
