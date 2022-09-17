pub mod local;
pub mod s3;

use async_trait::async_trait;
use tokio::fs;

use crate::config::StorageConfig;

use self::{local::LocalProvider, s3::S3Provider};

#[async_trait]
/// Base storage provider type
pub trait StorageProvider: Sync + Send {
    /// Put the object.
    async fn put_object(&self, name: &str, data: &Vec<u8>) -> Result<(), anyhow::Error>;

    /// Delete multiple objects.
    async fn delete_objects(&self, keys: Vec<String>) -> Result<(), anyhow::Error>;

    /// Get the buffer of the object.
    async fn get_object(&self, path: &str) -> Result<Vec<u8>, anyhow::Error>;
}

/// Create a new storage based on [`StorageConfig`].
pub async fn new_storage(config: StorageConfig) -> Box<dyn StorageProvider> {
    match config {
        StorageConfig::Local(v) => {
            if !v.path.exists() {
                fs::create_dir(&v.path).await.expect(&format!(
                    "Unable to create {} directory",
                    v.path.to_str().unwrap_or("storage")
                ));
            }

            // Thumbnail directory
            let mut thumb_path = v.path.clone();
            thumb_path.push("thumb");

            if !thumb_path.exists() {
                fs::create_dir(&thumb_path)
                    .await
                    .expect("Unable to create thumbnail directory");
            }

            Box::new(LocalProvider::new(v.path.clone()))
        }
        StorageConfig::S3(v) => Box::new(S3Provider::new(
            &v.bucket,
            &v.access_key,
            &v.secret_key,
            v.region.clone(),
        )),
    }
}
