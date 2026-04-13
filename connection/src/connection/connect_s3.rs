use std::error::Error;
use opendal::services::{S3};
use opendal::Operator;

/// Zero-sized handle for constructing S3 [`Operator`] instances.
#[derive(Clone)]
pub struct ConnectionS3;


impl ConnectionS3 {

    /// Builds a base [`S3`] builder pre-configured with the given endpoint and a fixed region.
    ///
    /// This is intentionally private — callers should use one of the typed `connect_*` variants
    /// that layer authentication on top.
    fn connect(endpoint: &str) -> Result<S3, Box<dyn Error>> {
        let mut builder = S3::default();
        builder.endpoint(endpoint);
        builder.region("us-east-1");
        Ok(builder)
    }

    /// Creates a fully authenticated [`Operator`] using access-key / secret-key credentials.
    ///
    /// # Arguments
    /// * `endpoint`   - The S3-compatible endpoint URL (e.g. `http://localhost:9000`).
    /// * `access_key` - The access key ID.
    /// * `secret_key` - The secret access key.
    /// * `bucket`     - The target bucket name.
    pub fn connect_md5(endpoint: &str, access_key: &str, secret_key: &str, bucket: &str) -> Result<Operator, Box<dyn Error>> {

        let builder = Self::connect(endpoint).map(|mut b| {
            b.access_key_id(access_key);
            b.secret_access_key(secret_key);
            b.bucket(bucket);
            b
        })?;

        let op = Operator::new(builder)?.finish();

        Ok(op)
    }

    /// Creates an [`Operator`] using STS temporary credentials (assumed IAM role or short-lived token).
    ///
    /// The session token is the third credential issued by AWS STS alongside the temporary
    /// access key and secret key. It must be included on every request — omitting it causes
    /// an `InvalidClientTokenId` error from S3.
    ///
    /// # Arguments
    /// * `endpoint`      - The S3-compatible endpoint URL (e.g. `https://s3.amazonaws.com`).
    /// * `access_key`    - The temporary access key ID issued by STS.
    /// * `secret_key`    - The temporary secret access key issued by STS.
    /// * `session_token` - The session token issued by STS.
    /// * `bucket`        - The target bucket name.
    pub fn connect_with_session_token(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        session_token: &str,
        bucket: &str,
    ) -> Result<Operator, Box<dyn Error>> {

        let builder = Self::connect(endpoint).map(|mut b| {
            b.access_key_id(access_key);
            b.secret_access_key(secret_key);
            b.security_token(session_token);
            b.bucket(bucket);
            b
        })?;

        let op = Operator::new(builder)?.finish();

        Ok(op)
    }

}
