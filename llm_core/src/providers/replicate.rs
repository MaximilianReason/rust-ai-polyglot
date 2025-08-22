#![cfg(feature = "provider-replicate")]
use async_trait::async_trait;
use crate::{Provider, types::ChatMessage};
use anyhow::Result;
use replicate_client as replicate;

pub struct ReplicateProvider { client: replicate::Client }

impl ReplicateProvider {
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("REPLICATE_API_TOKEN")?;
        Ok(Self { client: replicate::Client::new(&token)? })
    }
}

#[async_trait]
impl Provider for ReplicateProvider {
    async fn chat(&self, _messages: &[ChatMessage], _model: &str) -> Result<String> {
        // Not typically used for chat; prefer image/video generation or model-specific endpoints.
        Ok("Replicate chat placeholder".into())
    }
}
