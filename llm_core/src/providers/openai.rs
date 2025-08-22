#[cfg(feature = "provider-openai")]
use async_openai::{Client, config::{OpenAIConfig, AzureConfig}, types::*};
use async_trait::async_trait;
use crate::{Provider, types::{ChatMessage, ChatRole}};
use anyhow::Result;

pub struct OpenAIProvider { client: Client }

impl OpenAIProvider {
    pub fn new_openai() -> Self {
        // Uses OPENAI_API_KEY from env by default
        let cfg = OpenAIConfig::default();
        Self { client: Client::with_config(cfg) }
    }

    pub fn new_azure(api_base: &str, api_version: &str, api_key: &str) -> Self {
        let cfg = AzureConfig::new()
            .with_api_base(api_base)
            .with_api_version(api_version)
            .with_api_key(api_key);
        Self { client: Client::with_config(cfg) }
    }

    pub fn from_azure_env() -> Result<Self> {
        let base = std::env::var("AZURE_OPENAI_ENDPOINT")?;
        let ver  = std::env::var("AZURE_OPENAI_API_VERSION")?;
        let key  = std::env::var("AZURE_OPENAI_KEY")?;
        Ok(Self::new_azure(&base, &ver, &key))
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn chat(&self, messages: &[ChatMessage], model: &str) -> Result<String> {
        let msgs: Vec<ChatCompletionRequestMessage> = messages.iter().map(|m| {
            let role = match m.role {
                ChatRole::System => Role::System,
                ChatRole::Assistant => Role::Assistant,
                ChatRole::User => Role::User,
            };
            ChatCompletionRequestMessage {
                role,
                content: Some(m.content.clone().into()),
                name: None, tool_calls: None, function_call: None
            }
        }).collect();

        let req = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(msgs)
            .build()?;
        let resp = self.client.chat().create(req).await?;
        Ok(resp.choices.first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    async fn stream_chat(&self, messages: &[ChatMessage], model: &str) -> Result<()> {
        let msgs: Vec<ChatCompletionRequestMessage> = messages.iter().map(|m| {
            let role = match m.role {
                ChatRole::System => Role::System,
                ChatRole::Assistant => Role::Assistant,
                ChatRole::User => Role::User,
            };
            ChatCompletionRequestMessage {
                role,
                content: Some(m.content.clone().into()),
                name: None, tool_calls: None, function_call: None
            }
        }).collect();

        let req = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(msgs)
            .stream(true)
            .build()?;

        let mut stream = self.client.chat().create_stream(req).await?;
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await.transpose()? {
            if let Some(delta) = chunk.choices.first().and_then(|c| c.delta.content.clone()) {
                print!("{delta}");
            }
        }
        Ok(())
    }
}
