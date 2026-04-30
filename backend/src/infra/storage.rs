use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use uuid::Uuid;

pub struct Storage {
    client: S3Client,
    bucket: String,
    endpoint: String,
    public_endpoint: String,
}

impl Storage {
    pub async fn new(endpoint: String, region: String, bucket: String, public_endpoint: Option<String>) -> anyhow::Result<Self> {
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

        // Ensure bucket exists (ignoring errors in dev)
        match client.create_bucket().bucket(&bucket).send().await {
            Ok(_) => tracing::info!("Created S3 bucket: {}", bucket),
            Err(e) => tracing::warn!("S3 bucket may already exist: {e}"),
        }

        Ok(Self {
            client,
            bucket,
            endpoint,
            public_endpoint,
        })
    }

    pub async fn upload(
        &self,
        data: Vec<u8>,
        content_type: &str,
        prefix: &str,
    ) -> anyhow::Result<String> {
        let key = format!("{}/{}.jpg", prefix, Uuid::new_v4());

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(key)
    }

    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    pub fn public_url(&self, key: &str) -> String {
        format!("{}/{}/{}", self.public_endpoint, self.bucket, key)
    }
}
