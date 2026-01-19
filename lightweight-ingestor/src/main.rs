use polars::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Lightweight Ingestor (Polars) ---");
    
    // Example: Create a dummy DataFrame and write to Parquet
    let mut df = df!(
        "device_id" => &["SENSOR_01", "SENSOR_02"],
        "value" => &[10.5, 20.3]
    )?;

    println!("Original DataFrame:\n{}", df);

    let output_path = "output.parquet";
    let mut file = std::fs::File::create(output_path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;

    println!("Ingested data to {}", output_path);
    
    Ok(())
}
