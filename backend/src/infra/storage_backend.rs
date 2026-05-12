use async_trait::async_trait;

/// Abstract storage backend for photo uploads.
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Upload raw bytes and return a storage key.
    async fn upload(&self, data: Vec<u8>, content_type: &str, prefix: &str) -> anyhow::Result<String>;

    /// Delete an object by its storage key.
    async fn delete(&self, key: &str) -> anyhow::Result<()>;

    /// Generate a public URL for the given storage key.
    fn public_url(&self, key: &str) -> String;
}
