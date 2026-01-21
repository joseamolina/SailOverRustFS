use std::error::Error;
use opendal::services::{S3};
use opendal::Operator;

#[derive(Clone)]
pub struct ConnectionS3;


impl ConnectionS3 {

    /*
    Basic method for authentication
     */
    fn connect(endpoint: &str) -> Result<S3, Box<dyn Error>> {
        let mut builder = S3::default();
        builder.endpoint(endpoint);
        builder.region("us-east-1");
        Ok(builder)
    }

    /*
    Connection using Basic Authentication
     */
    pub fn connect_md5(endpoint: &str, access_key: &str, secret_key: &str, bucket: &str) -> Result<Operator, Box<dyn Error>> {

        let builder = ConnectionS3::connect(endpoint).map(|mut b| {
            b.access_key_id(access_key);
            b.secret_access_key(secret_key);
            b.bucket(bucket);
            b
        })?;

        let op = Operator::new(builder)?.finish();

        Ok(op)
    }

}
