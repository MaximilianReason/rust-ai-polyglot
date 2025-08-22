pub mod types;
pub mod providers;

pub use types::{ChatMessage, ChatRole, ProviderKind};

use async_trait::async_trait;

/// The unified interface every provider implements.
#[async_trait]
pub trait Provider: Send + Sync {
    async fn chat(&self, messages: &[ChatMessage], model: &str) -> anyhow::Result<String>;

    /// Stream tokens (print as they arrive, or forward to a callback in your app).
    async fn stream_chat(&self, _messages: &[ChatMessage], _model: &str) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("streaming not supported for this provider"))
    }

    /// Optional ops — providers may `Err` if unsupported.
    async fn embed(&self, _inputs: &[String], _model: &str) -> anyhow::Result<Vec<Vec<f32>>> {
        Err(anyhow::anyhow!("embeddings not supported"))
    }

    async fn image(&self, _prompt: &str, _model: &str) -> anyhow::Result<Vec<u8>> {
        Err(anyhow::anyhow!("image generation not supported"))
    }
}

