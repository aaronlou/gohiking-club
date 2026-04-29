use async_trait::async_trait;
use serde_json::Value;

use base64::Engine;

use super::{PhotoScorer, ScoreResult};

pub struct OpenAIScorer {
    api_key: String,
    model: String,
    max_tokens: u32,
    client: reqwest::Client,
}

impl OpenAIScorer {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4o".into(),
            max_tokens: 500,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[async_trait]
impl PhotoScorer for OpenAIScorer {
    fn name(&self) -> &'static str {
        "openai"
    }

    async fn score(&self, image_data: &[u8], _filename: &str) -> anyhow::Result<ScoreResult> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(image_data);

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": super::claude::PROMPT },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/jpeg;base64,{b64}")
                        }
                    }
                ]
            }]
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(super::claude::parse_score(&text))
    }
}
