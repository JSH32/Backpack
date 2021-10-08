use super::StorageProvider;
use async_trait::async_trait;
use infer;

use rusoto_core::{
    ByteStream, 
    HttpClient, 
    Region, 
    credential
};

use rusoto_s3::{
    DeleteObjectRequest, 
    PutObjectRequest,
    S3, 
    S3Client
};

pub struct S3Provider {
    bucket: String,
    client: S3Client
}

impl S3Provider {
    pub fn new(bucket: &str, access_key: &str, secret_key: &str, s3_region: Region) -> Self {
        let credential_provider = credential::StaticProvider::new_minimal(access_key.to_string(), secret_key.to_string());
        Self {
            client: S3Client::new_with(HttpClient::new().expect("S3 dispatcher could not be created"), credential_provider, s3_region),
            bucket: bucket.into()
        }
    }
}

#[async_trait]
impl StorageProvider for S3Provider {
    async fn put_object(&self, name: &str, data: &Vec<u8>) -> Result<(), String> {
        // Attempt to detect content type
        let content_type = match infer::get(&data) {
            Some(kind) => {
                Some(kind.mime_type().to_string())
            },
            None => None
        };

        match self.client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            body: Some(ByteStream::from(data.clone())),
            key: name.to_string(),
            acl: Some("public-read".into()),
            content_type: content_type,
            ..Default::default()
        }).await {
            Ok(_) => Ok(()),
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