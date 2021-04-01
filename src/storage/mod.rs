pub mod s3;

use async_trait::async_trait;

#[async_trait]
/// Base storage provider type
pub trait StorageProvider: Sync + Send {
    /// Put the object/file on the storage source
    async fn put_object(&self, name: &str, data: Vec<u8>) -> Result<String, String>;

    /// Delete the object/file on the storage source
    async fn delete_object(&self, name: &str) -> Result<(), String>;

    /// Get the base URL where files are located
    fn get_base(&self) -> String;
}