use crate::{Provider, types::*};
use anyhow::Result;

#[cfg(feature = "provider-openai")]
pub mod openai;
#[cfg(feature = "provider-anthropic")]
pub mod anthropic;
#[cfg(feature = "provider-openrouter")]
pub mod openrouter;
#[cfg(feature = "provider-hf")]
pub mod hf;
#[cfg(feature = "provider-replicate")]
pub mod replicate;
#[cfg(feature = "provider-gemini")]
pub mod gemini;

/// Simple factory — expand as needed.
pub fn make_provider(kind: ProviderKind) -> Result<Box<dyn Provider>> {
    match kind {
        #[cfg(feature = "provider-openai")]
        ProviderKind::OpenAI => Ok(Box::new(openai::OpenAIProvider::new_openai())),
        #[cfg(feature = "provider-openai")]
        ProviderKind::Azure => openai::OpenAIProvider::from_azure_env().map(|p| Box::new(p) as _),

        #[cfg(feature = "provider-anthropic")]
        ProviderKind::Anthropic => anthropic::AnthropicProvider::from_env().map(|p| Box::new(p) as _),

        #[cfg(feature = "provider-openrouter")]
        ProviderKind::OpenRouter => openrouter::OpenRouterProvider::from_env().map(|p| Box::new(p) as _),

        #[cfg(feature = "provider-hf")]
        ProviderKind::HuggingFace => hf::HuggingFaceProvider::from_env().map(|p| Box::new(p) as _),

        #[cfg(feature = "provider-replicate")]
        ProviderKind::Replicate => replicate::ReplicateProvider::from_env().map(|p| Box::new(p) as _),

        #[cfg(feature = "provider-gemini")]
        ProviderKind::Gemini => gemini::GeminiProvider::from_env().map(|p| Box::new(p) as _),

        #[allow(unreachable_patterns)]
        _ => anyhow::bail!("Provider feature not enabled")
    }
}
