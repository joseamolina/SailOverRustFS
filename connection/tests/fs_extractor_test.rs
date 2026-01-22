use connection::fs::fs_extractor::{FsExtractor};
use std::fs::{self, File as StdFile};
use std::io::Write;
use opendal::Operator;
use opendal::services::Fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_check_exists() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.txt");
    let mut file = StdFile::create(&file_path).unwrap();
    writeln!(file, "content").unwrap();

    let mut builder = Fs::default();
    builder.root(dir.path().to_str().unwrap());

    let op = Operator::new(builder)
        .expect("operator")
        .finish();



    println!("{}", FsExtractor::check_exists(file_path.to_str().unwrap(), op).await.unwrap());

    //assert!(FsExtractor::check_exists(file_path.to_str().unwrap(), op).await.unwrap());
    //assert!(!FsExtractor::check_exists("non_existent_file"));
}

/*
#[test]
#[ignore]
fn test_files_recursive() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    let file2 = subdir.join("file2.txt");

    StdFile::create(&file1).unwrap();
    StdFile::create(&file2).unwrap();

    let files = FsExtractor::files(dir.path().to_str().unwrap(), true);
    assert_eq!(files.len(), 2);
    assert!(files.iter().any(|f| f.contains("file1.txt")));
    assert!(files.iter().any(|f| f.contains("file2.txt")));
}

#[test]
#[ignore]
fn test_extract() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    StdFile::create(&file1).unwrap();

    let (files, resources) = FsExtractor::extract(dir.path().to_str().unwrap());
    
    assert_eq!(files.len(), 1);
    assert_eq!(resources.len(), 1);
    assert!(files[0].contains("file1.txt"));
    assert!(resources[0].path.contains("file1.txt"));
}
*/