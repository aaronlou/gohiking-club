use async_trait::async_trait;
use serde_json::Value;

use base64::Engine;

use super::{PhotoScorer, ScoreResult};

pub struct GeminiScorer {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiScorer {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gemini-2.0-flash".into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[async_trait]
impl PhotoScorer for GeminiScorer {
    fn name(&self) -> &'static str {
        "gemini"
    }

    async fn score(&self, image_data: &[u8], _filename: &str) -> anyhow::Result<ScoreResult> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(image_data);

        let body = serde_json::json!({
            "contents": [{
                "parts": [
                    { "text": super::claude::PROMPT },
                    {
                        "inlineData": {
                            "mimeType": "image/jpeg",
                            "data": b64
                        }
                    }
                ]
            }]
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model,
            self.api_key
        );

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error: {}", err_text);
        }

        let data: Value = resp.json().await?;
        let text = data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(super::claude::parse_score(&text))
    }
}
