use std::error::Error;
use opendal::Operator;
use connection::connection::connect_s3::ConnectionS3;

#[tokio::test]
pub async fn test_joke_endpoint_with_params() {
    let result: Result<Operator, Box<dyn Error>> = ConnectionS3::connect_md5(
        "http://127.0.0.1:9000",
        "rustfsadmin",
        "rustfsadmin",
        "testsail"
    );

    assert!(result.is_ok(), "Connection failed: {:?}", result.err());
    let dop = result.unwrap();

    dop.check().await.unwrap();

}
