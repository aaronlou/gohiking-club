pub mod traits;
pub mod claude;
pub mod openai;
pub mod ollama;
pub mod gemini;
pub mod registry;

pub use traits::*;
pub use registry::ScorerRegistry;
