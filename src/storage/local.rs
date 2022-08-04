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
    async fn put_object(&self, name: &str, data: &Vec<u8>) -> Result<(), anyhow::Error> {
        let mut path = self.path.clone();
        path.push(name);

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .await?;

        file.write_all(&data).await?;
        Ok(())
    }

    async fn delete_objects(&self, keys: Vec<String>) -> Result<(), anyhow::Error> {
        for key in keys {
            let mut path = self.path.clone();
            path.push(key);

            let _ = tokio::fs::remove_file(path).await;
        }

        Ok(())
    }

    async fn get_object(&self, path: &str) -> Result<Vec<u8>, anyhow::Error> {
        let mut path_buf = self.path.clone();
        path_buf.push(path);

        Ok(tokio::fs::read(path_buf).await?)
    }
}
