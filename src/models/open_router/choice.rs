use serde::{Deserialize, Serialize};

use super::ChatMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}
