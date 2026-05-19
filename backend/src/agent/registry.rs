use std::collections::HashMap;
use std::sync::Arc;

use super::LlmProvider;

/// Registry of available LLM chat providers.
///
/// Allows runtime switching between different LLM providers
/// without changing business logic. Uses `Arc` for thread-safe sharing.
pub struct LlmRegistry {
    providers: HashMap<String, Arc<dyn LlmProvider>>,
    active: String,
}

impl LlmRegistry {
    pub fn new(providers: HashMap<String, Arc<dyn LlmProvider>>, default_active: &str) -> Self {
        Self {
            providers,
            active: default_active.to_string(),
        }
    }

    /// Get a cloneable reference to the active LLM provider.
    pub fn active(&self) -> Arc<dyn LlmProvider> {
        self.providers
            .get(&self.active)
            .expect("active LLM provider should always exist")
            .clone()
    }

    /// Switch the active LLM provider by name.
    pub fn switch(&mut self, name: &str) -> anyhow::Result<()> {
        if self.providers.contains_key(name) {
            self.active = name.to_string();
            Ok(())
        } else {
            anyhow::bail!(
                "LLM provider '{}' not found, available: {:?}",
                name,
                self.available()
            )
        }
    }

    /// Current active provider name.
    pub fn active_name(&self) -> &str {
        &self.active
    }

    /// List all registered provider names.
    pub fn available(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}
