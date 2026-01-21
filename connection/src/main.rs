use std::error::Error;

mod connection;
use crate::connection::connect_s3::ConnectionS3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Data Validator ---");

    let _conn = ConnectionS3;

    Ok(())
}
