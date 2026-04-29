use async_trait::async_trait;
use base64::Engine;
use serde_json::Value;

use super::{PhotoScorer, ScoreDimensions, ScoreResult};

pub struct ClaudeScorer {
    api_key: String,
    model: String,
    max_tokens: u32,
    client: reqwest::Client,
}

impl ClaudeScorer {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-sonnet-4-20250506".into(),
            max_tokens: 500,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

#[async_trait]
impl PhotoScorer for ClaudeScorer {
    fn name(&self) -> &'static str {
        "claude"
    }

    async fn score(&self, image_data: &[u8], _filename: &str) -> anyhow::Result<ScoreResult> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(image_data);
        let media_type = "image/jpeg";

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": [{
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": PROMPT
                    },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": media_type,
                            "data": b64
                        }
                    }
                ]
            }]
        });

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        let text = data["content"][0]["text"]
            .as_str()
            .unwrap_or("failed to parse response")
            .to_string();

        Ok(parse_score(&text))
    }
}

pub(crate) const PROMPT: &str = r#"You are a photo quality evaluator for a hiking community website.
Analyze this hiking/outdoor photo and return a JSON object with scores (0-100) for:
- composition: framing, rule of thirds, visual balance
- lighting: exposure, contrast, golden hour quality
- clarity: sharpness, focus, resolution
- subject_interest: how engaging is the subject/scene
- overall: weighted total score

Return ONLY valid JSON:
{ "composition": 85, "lighting": 72, "clarity": 90, "subject_interest": 88, "overall": 84 }"#;

pub(crate) fn parse_score(text: &str) -> ScoreResult {
    // Try to extract JSON from the response
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            let json_str = &text[start..=end];
            if let Ok(v) = serde_json::from_str::<Value>(json_str) {
                let extract = |key: &str| -> f64 {
                    v[key].as_f64().unwrap_or(0.0).clamp(0.0, 100.0)
                };
                return ScoreResult {
                    overall: extract("overall"),
                    dimensions: ScoreDimensions {
                        composition: extract("composition"),
                        lighting: extract("lighting"),
                        clarity: extract("clarity"),
                        subject_interest: extract("subject_interest"),
                    },
                    raw_feedback: json_str.to_string(),
                };
            }
        }
    }
    ScoreResult {
        overall: 0.0,
        dimensions: ScoreDimensions {
            composition: 0.0,
            lighting: 0.0,
            clarity: 0.0,
            subject_interest: 0.0,
        },
        raw_feedback: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_score_valid() {
        let result = parse_score(r#"{"composition":85,"lighting":72,"clarity":90,"subject_interest":88,"overall":84}"#);
        assert!((result.overall - 84.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_score_with_markdown() {
        let result = parse_score("Here is the analysis:\n```json\n{\"composition\": 90, \"lighting\": 80, \"clarity\": 95, \"subject_interest\": 85, \"overall\": 88}\n```");
        assert!((result.overall - 88.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_score_fallback() {
        let result = parse_score("I cannot analyze this image");
        assert!((result.overall - 0.0).abs() < 0.1);
    }
}
