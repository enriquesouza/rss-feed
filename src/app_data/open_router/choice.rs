use serde::{Deserialize, Serialize};

use super::ChatMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
    pub native_finish_reason: Option<String>,
    pub index: Option<i32>,
    pub logprobs: Option<serde_json::Value>,
}
