use connection::fs::fs_extractor::FsExtractor;
use opendal::services::Fs;
use opendal::Operator;
use std::fs;
use std::fs::File as StdFile;
use std::io::Write;
use tempfile::tempdir;

/// Builds a local-filesystem [`Operator`] rooted at `root`.
/// All paths passed to the operator must be relative to that root.
fn local_operator(root: &str) -> Operator {
    let mut builder = Fs::default();
    builder.root(root);
    Operator::new(builder).expect("failed to build Fs operator").finish()
}

// ── check_exists ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn check_exists_returns_true_for_existing_file() {
    let dir = tempdir().unwrap();
    let mut file = StdFile::create(dir.path().join("test_file.txt")).unwrap();
    writeln!(file, "content").unwrap();

    let op = local_operator(dir.path().to_str().unwrap());

    // Path must be relative to the operator root.
    assert!(FsExtractor::check_exists("test_file.txt", op).await.unwrap());
}

#[tokio::test]
async fn check_exists_returns_false_for_missing_file() {
    let dir = tempdir().unwrap();
    let op = local_operator(dir.path().to_str().unwrap());

    assert!(!FsExtractor::check_exists("nonexistent.txt", op).await.unwrap());
}

// ── files ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn files_lists_files_at_root() {
    let dir = tempdir().unwrap();
    StdFile::create(dir.path().join("a.txt")).unwrap();
    StdFile::create(dir.path().join("b.txt")).unwrap();

    let op = local_operator(dir.path().to_str().unwrap());

    // "/" lists the root of the operator.
    let files = FsExtractor::files("/", op).await;

    assert_eq!(files.len(), 2);
    assert!(files.iter().any(|f| f.contains("a.txt")));
    assert!(files.iter().any(|f| f.contains("b.txt")));
}

#[tokio::test]
async fn files_excludes_directories() {
    let dir = tempdir().unwrap();
    StdFile::create(dir.path().join("file.txt")).unwrap();
    fs::create_dir(dir.path().join("subdir")).unwrap();

    let op = local_operator(dir.path().to_str().unwrap());

    let files = FsExtractor::files("/", op).await;

    // Only the regular file should appear, not the directory.
    assert_eq!(files.len(), 1);
    assert!(files[0].contains("file.txt"));
}

#[tokio::test]
async fn files_returns_empty_for_empty_directory() {
    let dir = tempdir().unwrap();
    let op = local_operator(dir.path().to_str().unwrap());

    let files = FsExtractor::files("/", op).await;

    assert!(files.is_empty());
}

// ── extract ───────────────────────────────────────────────────────────────────

#[test]
fn extract_returns_files_and_resources() {
    let dir = tempdir().unwrap();
    StdFile::create(dir.path().join("file1.txt")).unwrap();
    StdFile::create(dir.path().join("file2.txt")).unwrap();

    let (files, resources) = FsExtractor::extract(dir.path().to_str().unwrap());

    assert_eq!(files.len(), 2);
    assert_eq!(resources.len(), 2);
    assert!(files.iter().any(|f| f.contains("file1.txt")));
    assert!(files.iter().any(|f| f.contains("file2.txt")));
}

#[test]
fn extract_resource_paths_match_file_paths() {
    let dir = tempdir().unwrap();
    StdFile::create(dir.path().join("data.txt")).unwrap();

    let (files, resources) = FsExtractor::extract(dir.path().to_str().unwrap());

    assert_eq!(files.len(), 1);
    assert_eq!(resources.len(), 1);
    assert_eq!(files[0], resources[0].path);
}

#[test]
fn extract_walks_subdirectories() {
    let dir = tempdir().unwrap();
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    StdFile::create(dir.path().join("root.txt")).unwrap();
    StdFile::create(subdir.join("nested.txt")).unwrap();

    let (files, resources) = FsExtractor::extract(dir.path().to_str().unwrap());

    assert_eq!(files.len(), 2);
    assert_eq!(resources.len(), 2);
    assert!(files.iter().any(|f| f.contains("root.txt")));
    assert!(files.iter().any(|f| f.contains("nested.txt")));
}

#[test]
fn extract_returns_empty_for_nonexistent_path() {
    let (files, resources) = FsExtractor::extract("/nonexistent/path/that/does/not/exist");

    assert!(files.is_empty());
    assert!(resources.is_empty());
}

#[test]
fn extract_returns_empty_for_empty_directory() {
    let dir = tempdir().unwrap();

    let (files, resources) = FsExtractor::extract(dir.path().to_str().unwrap());

    assert!(files.is_empty());
    assert!(resources.is_empty());
}
