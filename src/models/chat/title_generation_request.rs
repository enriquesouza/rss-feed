use serde::{Deserialize, Serialize};
use validator::Validate;

use super::LastMessageDTO;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TitleGenerationRequest {
    #[validate(length(min = 1, message = "At least one message is required"))]
    #[serde(rename = "lastMessages")]
    pub last_messages: Vec<LastMessageDTO>,
    pub model: Option<String>,
    #[serde(rename = "userLanguage")]
    pub user_language: Option<String>,
}
