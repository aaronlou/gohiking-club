use std::time::Duration;

use async_trait::async_trait;
use base64::Engine;
use serde_json::Value;

use super::{PhotoScorer, ScoreDimensions, ScoreResult};

const GEMINI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-lite:generateContent";

const PROMPT: &str = r#"你是一位专业摄影评论家。请仔细分析这张照片的实际内容，给出真实的评价。不要给安全分，不要对所有照片都给相似分数。

请给出：
1. 0.0 到 5.0 的评分（一位小数即可，如 2.5、4.2）
2. 一段 50-100 字的中文点评，具体指出这张照片的构图、光线、色彩、主体表达等方面的真实优缺点。

请严格按照以下 JSON 格式回复：
{"score": X.X, "review": "你的点评内容"}

评分参考标准（根据照片实际质量灵活使用，不要机械套用）：
- 4.5-5.0：极少数的杰作。具有强烈的视觉冲击力和艺术独创性。
- 3.8-4.4：优秀作品。技术成熟，有明显的艺术表达，让人印象深刻。
- 3.0-3.7：合格作品。有基本美感，能看到拍摄者的用心，但有改进空间。
- 2.0-2.9：有明显短板。构图、曝光、对焦或主体表达上存在需要正视的问题。
- 1.0-1.9：问题较多。多个维度表现不佳，需要系统学习基础。
- 0.0-0.9：极少数的失误。存在导致照片无法观看的致命错误。

最重要的要求：
1. 分数必须反映这张照片的真实质量，不要给"安全分"。
2. 点评必须与分数一致：高分就真诚夸奖优点，低分就指出具体问题。
3. 不同照片应该有不同的分数，哪怕是细微差异。"#;

pub struct GeminiScorer {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiScorer {
    pub fn new(api_key: String) -> Self {
        let mut client_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(30));

        if let Ok(proxy_url) =
            std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy"))
        {
            if let Ok(proxy) = reqwest::Proxy::https(&proxy_url) {
                client_builder = client_builder.proxy(proxy);
                tracing::info!("Using HTTPS proxy for Gemini API");
            }
        }

        Self {
            api_key,
            model: "gemini-3.1-flash-lite".into(),
            client: client_builder.build().expect("Failed to build HTTP client for Gemini"),
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
                    { "text": PROMPT },
                    {
                        "inline_data": {
                            "mime_type": "image/jpeg",
                            "data": b64
                        }
                    }
                ]
            }]
        });

        let url = format!("{}?key={}", GEMINI_API_URL, self.api_key);

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error ({}): {}", status, text);
        }

        let data: Value = resp.json().await?;
        let parsed = GeminiResponse::from_raw(&data)?;
        Ok(parsed.to_score_result())
    }
}

/// ACL: isolates Gemini's external response schema from the domain.
struct GeminiResponse {
    score: f64,
    review: String,
}

impl GeminiResponse {
    fn from_raw(value: &Value) -> anyhow::Result<Self> {
        let text = value["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("{}");

        let json_text = extract_json_from_markdown(text);

        let parsed: Value = serde_json::from_str(&json_text)
            .map_err(|e| anyhow::anyhow!("JSON parse error: {} | text: {}", e, text))?;

        let score = parsed["score"].as_f64().unwrap_or(3.0);
        let review = parsed["review"]
            .as_str()
            .unwrap_or("No review available")
            .to_string();

        Ok(Self { score, review })
    }

    fn to_score_result(self) -> ScoreResult {
        // Map 0-5 scale to 0-100 for compatibility with the existing scoring system
        let overall = (self.score * 20.0).clamp(0.0, 100.0);
        ScoreResult {
            overall,
            dimensions: ScoreDimensions {
                composition: overall,
                lighting: overall,
                clarity: overall,
                subject_interest: overall,
            },
            raw_feedback: self.review,
        }
    }
}

/// Strip ```json fences or ``` blocks from the response text.
fn extract_json_from_markdown(text: &str) -> String {
    if let Some(inner) = text
        .split("```json")
        .nth(1)
        .and_then(|s| s.split("```").next())
    {
        inner.trim().to_string()
    } else if let Some(inner) = text.split("```").nth(1) {
        inner.trim().to_string()
    } else {
        text.trim().to_string()
    }
}
