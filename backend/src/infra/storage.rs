use async_trait::async_trait;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use uuid::Uuid;

use super::storage_backend::StorageBackend;

pub struct S3Storage {
    client: S3Client,
    bucket: String,
    public_endpoint: String,
}

impl S3Storage {
    pub async fn new(
        endpoint: String,
        region: String,
        bucket: String,
        public_endpoint: Option<String>,
    ) -> anyhow::Result<Self> {
        let public_endpoint = public_endpoint.unwrap_or_else(|| endpoint.clone());

        let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .endpoint_url(&endpoint)
            .region(Region::new(region))
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();

        let client = S3Client::from_conf(s3_config);

        // Ensure bucket exists
        match client.create_bucket().bucket(&bucket).send().await {
            Ok(_) => tracing::info!("Created S3 bucket: {bucket}"),
            Err(e) => tracing::warn!("S3 bucket may already exist: {e}"),
        }

        Ok(Self { client, bucket, public_endpoint })
    }
}

#[async_trait]
impl StorageBackend for S3Storage {
    async fn upload(&self, data: Vec<u8>, _content_type: &str, prefix: &str) -> anyhow::Result<String> {
        let key = format!("{}/{}.jpg", prefix, Uuid::new_v4());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type(_content_type)
            .send()
            .await?;

        Ok(key)
    }

    async fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    fn public_url(&self, key: &str) -> String {
        format!("{}/{}/{}", self.public_endpoint, self.bucket, key)
    }
}
