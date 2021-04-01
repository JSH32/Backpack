use async_trait::async_trait;
use rusoto_s3::{*};
use rusoto_core::{*};
use infer;

pub struct Storage {
    base: String,
    bucket: String,
    client: S3Client
}

#[async_trait]
pub trait StorageSource {
    /// Put the object/file on the storage source
    async fn put_object(&self, name: &str, data: Vec<u8>) -> Result<String, String>;

    /// Delete the object/file on the storage source
    async fn delete_object(&self, name: &str) -> Result<(), String>;

    /// Get the base URL where files are located
    fn get_base(&self) -> String;
}

impl Storage {
    pub fn new(base: &str, bucket: &str, access_key: &str, secret_key: &str, s3_region: Region) -> Self {
        let credential_provider = credential::StaticProvider::new_minimal(access_key.to_string(), secret_key.to_string());
        Self {
            base: base.to_string(),
            client: S3Client::new_with(HttpClient::new().expect("S3 dispatcher could not be created"), credential_provider, s3_region),
            bucket: bucket.into()
        }
    }
    /// Put a file object on the data store
    pub async fn put_object(&self, name: &str, data: Vec<u8>) -> Result<(), RusotoError<PutObjectError>> {
        // Attempt to detect content type
        let content_type = match infer::get(&data) {
            Some(kind) => {
                Some(kind.mime_type().to_string())
            },
            None => None
        };

        // Upload to S3 API
        self.client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            body: Some(ByteStream::from(data)),
            key: name.to_string(),
            acl: Some("public-read".into()),
            content_type: content_type,
            ..Default::default()
        }).await?;

        Ok(())
    }
    pub async fn delete_object(&self, name: &str) -> Result<(), RusotoError<DeleteObjectError>> {
        self.client.delete_object(DeleteObjectRequest {
            bucket: self.bucket.clone(),
            key: name.to_string(),
            ..Default::default()
        }).await?;

        Ok(())
    }
    /// Get the base URL where files are stored
    pub fn get_base(&self) -> String {
        self.base
    }
}