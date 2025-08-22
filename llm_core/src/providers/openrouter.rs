#![cfg(feature = "provider-openrouter")]
use openrouter_api::{OpenRouterClient, Ready, Result as ORResult};
use openrouter_api::types::chat::{ChatCompletionRequest, Message};
use async_trait::async_trait;
use crate::{Provider, types::{ChatMessage, ChatRole}};
use anyhow::Result;

pub struct OpenRouterProvider { client: OpenRouterClient<Ready> }

impl OpenRouterProvider {
    pub fn from_env() -> Result<Self> {
        Ok(Self { client: OpenRouterClient::from_env()? })
    }
}

#[async_trait]
impl Provider for OpenRouterProvider {
    async fn chat(&self, messages: &[ChatMessage], model: &str) -> Result<String> {
        let req = ChatCompletionRequest {
            model: model.to_string(),
            messages: messages.iter().map(|m| {
                let role = match m.role { ChatRole::System=>"system", ChatRole::Assistant=>"assistant", ChatRole::User=>"user" };
                Message::new(role, &m.content)
            }).collect(),
            ..Default::default()
        };
        let resp = self.client.chat().create(req).await?;
        Ok(resp.choices[0].message.content.clone().unwrap_or_default())
    }
}
