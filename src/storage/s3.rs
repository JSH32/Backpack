use rusoto_s3::{*};
use rusoto_core::{*};
use infer;
use async_trait::async_trait;

use super::StorageProvider;

pub struct S3Provider {
    base: String,
    bucket: String,
    client: S3Client
}

impl S3Provider {
    pub fn new(base: &str, bucket: &str, access_key: &str, secret_key: &str, s3_region: Region) -> Self {
        let credential_provider = credential::StaticProvider::new_minimal(access_key.to_string(), secret_key.to_string());
        Self {
            base: base.to_string(),
            client: S3Client::new_with(HttpClient::new().expect("S3 dispatcher could not be created"), credential_provider, s3_region),
            bucket: bucket.into()
        }
    }
}

#[async_trait]
impl StorageProvider for S3Provider {
    fn get_base(&self) -> String {
        self.base.clone()
    }
    
    async fn put_object(&self, name: &str, data: Vec<u8>) -> Result<String, String> {
        // Attempt to detect content type
        let content_type = match infer::get(&data) {
            Some(kind) => {
                Some(kind.mime_type().to_string())
            },
            None => None
        };

        match self.client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            body: Some(ByteStream::from(data)),
            key: name.to_string(),
            acl: Some("public-read".into()),
            content_type: content_type,
            ..Default::default()
        }).await {
            Ok(_) => Ok(format!("{}/{}", self.base, name)),
            Err(err) => Err(err.to_string())
        }
    }

    async fn delete_object(&self, name: &str) -> Result<(), String> {
        match self.client.delete_object(DeleteObjectRequest {
            bucket: self.bucket.clone(),
            key: name.to_string(),
            ..Default::default()
        }).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}