use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{CompressedDocumentDTO, LastMessageDTO, SkillDTO, TypeChatDTO};

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct MessageDTO {
    #[validate(length(min = 1, message = "Question cannot be empty"))]
    pub question: String,
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(rename = "isAudioMessage", default)]
    pub is_audio_message: bool,
    #[serde(rename = "typeChat")]
    pub type_chat: Option<TypeChatDTO>,
    #[serde(rename = "lastMessages", default)]
    pub last_messages: Vec<LastMessageDTO>,
    #[serde(rename = "enableWebSearch", default)]
    pub enable_web_search: bool,
    #[serde(rename = "compressedDocuments")]
    pub compressed_documents: Option<Vec<CompressedDocumentDTO>>,
    #[serde(rename = "userLanguage")]
    pub user_language: Option<String>,
    #[serde(rename = "workspacePath")]
    pub workspace_path: Option<String>,
    #[serde(rename = "profileImage")]
    pub profile_image: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "assistantName")]
    pub assistant_name: Option<String>,
    #[serde(rename = "assistantRole")]
    pub assistant_role: Option<String>,
    #[serde(rename = "assistantPrompt")]
    pub assistant_prompt: Option<String>,
    #[serde(default)]
    pub skills: Option<Vec<SkillDTO>>,
    #[serde(rename = "isFreeModel", default)]
    pub is_free_model: bool,
}
