use async_trait::async_trait;
use serde_json::Value;

use base64::Engine;

use super::{PhotoScorer, ScoreResult};

pub struct OllamaScorer {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaScorer {
    pub fn new(endpoint: String, model: String) -> Self {
        Self {
            endpoint,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl PhotoScorer for OllamaScorer {
    fn name(&self) -> &'static str {
        "ollama"
    }

    async fn score(&self, image_data: &[u8], _filename: &str) -> anyhow::Result<ScoreResult> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(image_data);

        let body = serde_json::json!({
            "model": self.model,
            "prompt": format!("{}\n\nReturn only valid JSON.", super::claude::PROMPT),
            "images": [b64],
            "stream": false,
            "format": "json"
        });

        let resp = self
            .client
            .post(format!("{}/api/generate", self.endpoint.trim_end_matches('/')))
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        let text = data["response"].as_str().unwrap_or("").to_string();
        Ok(super::claude::parse_score(&text))
    }
}
