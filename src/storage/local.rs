use std::path::PathBuf;

use super::StorageProvider;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

pub struct LocalProvider {
    path: PathBuf,
}

impl LocalProvider {
    pub fn new(path: PathBuf) -> Self {
        LocalProvider { path: path }
    }
}

#[async_trait]
impl StorageProvider for LocalProvider {
    async fn put_object(&self, name: &str, data: &Vec<u8>) -> Result<(), String> {
        let mut path = self.path.clone();
        path.push(name);

        if path.exists() {
            return Err("Path exists".to_string());
        }

        let mut file = match tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .await
        {
            Err(err) => return Err(err.to_string()),
            Ok(file) => file,
        };

        match file.write_all(&data).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn delete_object(&self, name: &str) -> Result<(), String> {
        let mut path = self.path.clone();
        path.push(name);

        match tokio::fs::remove_file(path).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
