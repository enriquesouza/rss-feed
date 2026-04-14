use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{ChatMessage, StreamOptions, ToolDefinition, UsageConfig};

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ChatRequest {
    #[validate(length(min = 1, message = "Model ID is required"))]
    pub model: String,
    #[validate(length(min = 1, message = "At least one message is required"))]
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0.0, max = 2.0, message = "Temperature must be between 0 and 2"))]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1, message = "Max tokens must be at least 1"))]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
