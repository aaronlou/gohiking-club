use serde::{Deserialize, Serialize};

/// Multi-dimensional scoring result from AI photo analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub overall: f64,
    pub dimensions: ScoreDimensions,
    pub raw_feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDimensions {
    pub composition: f64,
    pub lighting: f64,
    pub clarity: f64,
    pub subject_interest: f64,
}

/// Unified trait for AI photo scoring providers.
///
/// Implement this trait to add a new AI model provider.
/// See `ClaudeScorer`, `OpenAIScorer`, or `OllamaScorer` for examples.
#[async_trait::async_trait]
pub trait PhotoScorer: Send + Sync {
    /// Analyze and score a photo from raw image bytes.
    async fn score(&self, image_data: &[u8], filename: &str) -> anyhow::Result<ScoreResult>;

    /// Name of this scorer (for logging and debugging).
    fn name(&self) -> &'static str;
}
