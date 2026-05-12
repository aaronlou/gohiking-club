use async_trait::async_trait;
use std::path::PathBuf;
use uuid::Uuid;

use super::storage_backend::StorageBackend;

pub struct LocalStorage {
    base_dir: PathBuf,
    public_url_prefix: String,
}

impl LocalStorage {
    pub fn new(base_dir: impl Into<PathBuf>, public_url_prefix: impl Into<String>) -> anyhow::Result<Self> {
        let base_dir = base_dir.into();
        std::fs::create_dir_all(&base_dir)?;
        Ok(Self {
            base_dir,
            public_url_prefix: public_url_prefix.into(),
        })
    }

    fn path_for_key(&self, key: &str) -> PathBuf {
        self.base_dir.join(key)
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn upload(&self, data: Vec<u8>, _content_type: &str, prefix: &str) -> anyhow::Result<String> {
        let key = format!("{}/{}.jpg", prefix, Uuid::new_v4());
        let path = self.path_for_key(&key);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, data)?;
        Ok(key)
    }

    async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let path = self.path_for_key(key);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    fn public_url(&self, key: &str) -> String {
        format!("{}/{}", self.public_url_prefix.trim_end_matches('/'), key)
    }
}
