use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Data Validator ---");
    // Placeholder path
    let path = "test.parquet";
    
    if Path::new(path).exists() {
        validate_parquet(path)?;
    } else {
        println!("Please provide a valid parquet file path.");
    }
    
    Ok(())
}

fn validate_parquet(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let metadata = reader.metadata();
    
    println!("Validating: {}", path);
    println!("Number of rows: {}", metadata.file_metadata().num_rows());
    println!("Number of row groups: {}", metadata.num_row_groups());
    
    Ok(())
}
