use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub auth: AuthConfig,
    pub ai_scoring: AiScoringConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expires_in_secs: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub backend: String,
    pub s3: Option<S3Config>,
    pub veimagex: Option<VeImageXConfig>,
    pub local: Option<LocalConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    pub bucket: String,
    pub endpoint: String,
    pub region: String,
    pub public_endpoint: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VeImageXConfig {
    pub service_id: String,
    pub domain: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocalConfig {
    pub base_dir: String,
    pub public_url_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiScoringConfig {
    pub active: String,
    pub threshold: f64,
    pub concurrent_limit: usize,
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub model: String,
    pub max_tokens: Option<u32>,
    pub endpoint: Option<String>,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let mut builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::default().prefix("APP").separator("__"));

        // Inject DATABASE_URL from env if set
        if let Ok(url) = std::env::var("DATABASE_URL") {
            builder = builder.set_override("database.url", url)?;
        }

        Ok(builder.build()?.try_deserialize()?)
    }
}
