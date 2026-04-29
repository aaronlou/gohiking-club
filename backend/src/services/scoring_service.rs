use crate::ai::{ScoreResult, ScorerRegistry};

/// Orchestrates AI scoring with concurrency control and threshold management.
pub struct ScoringService {
    registry: ScorerRegistry,
    threshold: f64,
    semaphore: tokio::sync::Semaphore,
}

impl ScoringService {
    pub fn new(registry: ScorerRegistry, threshold: f64, concurrent_limit: usize) -> Self {
        Self {
            registry,
            threshold,
            semaphore: tokio::sync::Semaphore::new(concurrent_limit),
        }
    }

    /// Score a photo, respecting the concurrency limit.
    pub async fn score(&self, image_data: &[u8]) -> anyhow::Result<ScoreResult> {
        let _permit = self.semaphore.acquire().await?;
        self.registry.score(image_data, "upload.jpg").await
    }

    /// The current scoring threshold.
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// The name of the active scorer.
    pub fn active_scorer(&self) -> &str {
        self.registry.active_name()
    }

    /// Switch the active scorer at runtime.
    pub fn switch_scorer(&mut self, name: &str) -> anyhow::Result<()> {
        self.registry.switch(name)
    }
}
