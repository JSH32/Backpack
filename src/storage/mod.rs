pub mod local;
pub mod s3;

use async_trait::async_trait;

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
