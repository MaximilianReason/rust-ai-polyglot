#![cfg(feature = "provider-hf")]
use async_trait::async_trait;
use crate::{Provider, types::ChatMessage};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct HuggingFaceProvider { http: Client, token: String, model: String }

impl HuggingFaceProvider {
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("HF_TOKEN")?;
        let model = std::env::var("HF_MODEL").unwrap_or_else(|_| "distilbert-base-uncased".into());
        Ok(Self { http: Client::new(), token, model })
    }
}

#[async_trait]
impl Provider for HuggingFaceProvider {
    async fn chat(&self, messages: &[ChatMessage], _model: &str) -> Result<String> {
        let input = messages.iter().map(|m| &m.content).cloned().collect::<Vec<_>>().join("\n");
        let resp = self.http.post(format!("https://api-inference.huggingface.co/models/{}", self.model))
            .bearer_auth(&self.token)
            .json(&json!({ "inputs": input }))
            .send().await?
            .text().await?;
        Ok(resp)
    }
}
