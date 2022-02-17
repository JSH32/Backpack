pub mod local;
pub mod s3;

use async_trait::async_trait;

#[async_trait]
/// Base storage provider type
pub trait StorageProvider: Sync + Send {
    /// Put the object/file on the storage source
    async fn put_object(&self, name: &str, data: &Vec<u8>) -> Result<(), anyhow::Error>;

    /// Delete the object/file on the storage source
    async fn delete_object(&self, name: &str) -> Result<(), anyhow::Error>;

    async fn get_object(&self, path: &str) -> Result<Vec<u8>, anyhow::Error>;
}
