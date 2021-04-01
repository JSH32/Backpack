use super::StorageProvider;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

pub struct LocalProvider {
    path: String
}

impl LocalProvider {
    pub fn new(path: &str) -> Self {
        LocalProvider {
            path: path.to_string()
        }
    }
}

#[async_trait]
impl StorageProvider for LocalProvider {
    async fn put_object(&self, name: &str, data: Vec<u8>) -> Result<(), String> {
        let mut file = match tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}/{}", self.path, &name))
            .await {
                Err(err) => return Err(err.to_string()),
                Ok(file) => file
            };

        match file.write_all(&data).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }

    async fn delete_object(&self, name: &str) -> Result<(), String> {
        match tokio::fs::remove_file(format!("{}/{}", self.path, &name)).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}