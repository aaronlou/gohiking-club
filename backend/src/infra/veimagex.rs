use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use super::storage_backend::StorageBackend;

type HmacSha256 = Hmac<Sha256>;

pub struct VeImageXStorage {
    service_id: String,
    domain: String,
    access_key: String,
    secret_key: String,
}

impl VeImageXStorage {
    pub fn new(
        service_id: String,
        domain: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            service_id,
            domain,
            access_key,
            secret_key,
        }
    }

    fn _sign(&self, _payload: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(_payload.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        hex::encode(bytes)
    }
}

#[async_trait]
impl StorageBackend for VeImageXStorage {
    async fn upload(&self, data: Vec<u8>, _content_type: &str, prefix: &str) -> anyhow::Result<String> {
        let key = format!("{}/{}.jpg", prefix, Uuid::new_v4());
        // Placeholder: real implementation would call veImageX upload API
        tracing::info!("VeImageX upload placeholder for key: {}", key);
        let _ = data;
        Ok(key)
    }

    async fn delete(&self, key: &str) -> anyhow::Result<()> {
        tracing::info!("VeImageX delete placeholder for key: {}", key);
        Ok(())
    }

    fn public_url(&self, key: &str) -> String {
        format!("https://{}/{}/{}", self.domain, self.service_id, key)
    }
}
