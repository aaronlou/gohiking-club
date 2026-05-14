use std::collections::HashMap;

use super::PhotoScorer;

/// Registry of available photo scorers.
///
/// Allows runtime switching between different AI models
/// without changing business logic.
pub struct ScorerRegistry {
    providers: HashMap<String, Box<dyn PhotoScorer>>,
    active: String,
}

impl ScorerRegistry {
    /// Create a new registry with the given providers.
    /// `default_active` specifies which provider is active by default.
    pub fn new(
        providers: HashMap<String, Box<dyn PhotoScorer>>,
        default_active: &str,
    ) -> Self {
        Self {
            providers,
            active: default_active.to_string(),
        }
    }

    /// Get a reference to the active scorer.
    pub fn active(&self) -> &dyn PhotoScorer {
        self.providers
            .get(&self.active)
            .expect("active scorer should always exist")
            .as_ref()
    }

    /// Switch the active scorer by name.
    /// Returns an error if the name is not registered.
    pub fn switch(&mut self, name: &str) -> anyhow::Result<()> {
        if self.providers.contains_key(name) {
            self.active = name.to_string();
            Ok(())
        } else {
            anyhow::bail!("scorer '{}' not found, available: {:?}", name, self.available())
        }
    }

    /// Current active scorer name.
    pub fn active_name(&self) -> &str {
        &self.active
    }

    /// List all registered scorer names.
    pub fn available(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Score a photo using the active scorer.
    pub async fn score(&self, image_data: &[u8], filename: &str) -> anyhow::Result<super::ScoreResult> {
        self.active().score(image_data, filename).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::ai::{PhotoScorer, ScoreDimensions, ScoreResult};

    struct MockScorer(&'static str);

    #[async_trait]
    impl PhotoScorer for MockScorer {
        fn name(&self) -> &'static str { self.0 }
        async fn score(&self, _: &[u8], _: &str) -> anyhow::Result<ScoreResult> {
            Ok(ScoreResult {
                overall: 80.0,
                dimensions: ScoreDimensions { composition: 80.0, lighting: 80.0, clarity: 80.0, subject_interest: 80.0 },
                raw_feedback: "mock".into(),
            })
        }
    }

    #[test]
    fn test_switch_scorer() {
        let mut providers: HashMap<String, Box<dyn PhotoScorer>> = HashMap::new();
        providers.insert("a".into(), Box::new(MockScorer("a")));
        providers.insert("b".into(), Box::new(MockScorer("b")));

        let mut reg = ScorerRegistry::new(providers, "a");
        assert_eq!(reg.active_name(), "a");

        reg.switch("b").unwrap();
        assert_eq!(reg.active_name(), "b");

        assert!(reg.switch("nonexistent").is_err());
    }
}
