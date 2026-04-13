use std::error::Error;

mod connection;
use crate::connection::connect_s3::ConnectionS3;

/// Application entry point.
///
/// Initialises the S3 connection handle and delegates to the appropriate
/// validation or extraction workflow.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Data Validator ---");

    // Placeholder: build and use a ConnectionS3 instance to connect to S3.
    let _conn = ConnectionS3;

    Ok(())
}
