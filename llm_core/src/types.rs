use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatRole { System, User, Assistant }
impl From<&str> for ChatRole {
    fn from(s: &str) -> Self {
        match s {
            "system" => ChatRole::System,
            "assistant" => ChatRole::Assistant,
            _ => ChatRole::User
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage {
    pub fn system<S: Into<String>>(s: S) -> Self { Self { role: ChatRole::System, content: s.into() } }
    pub fn user<S: Into<String>>(s: S) -> Self { Self { role: ChatRole::User, content: s.into() } }
    pub fn assistant<S: Into<String>>(s: S) -> Self { Self { role: ChatRole::Assistant, content: s.into() } }
}

#[derive(Clone, Debug)]
pub enum ProviderKind {
    OpenAI,
    Azure,
    Anthropic,
    OpenRouter,
    HuggingFace,
    Replicate,
    Gemini,
}
