use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LastMessageDTO {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompressedDocumentDTO {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeChatDTO {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub prefix: Option<String>,
}

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

    // Assistant context sent inline from frontend (IndexedDB)
    // Replaces backend Solana PDA lookup
    #[serde(rename = "assistantName")]
    pub assistant_name: Option<String>,

    #[serde(rename = "assistantRole")]
    pub assistant_role: Option<String>,

    #[serde(rename = "assistantPrompt")]
    pub assistant_prompt: Option<String>,

    /// Skills attached to the assistant (prompt-type or tool-type)
    #[serde(default)]
    pub skills: Option<Vec<SkillDTO>>,

    /// Free model flag — sent by frontend based on cached pricing data
    #[serde(rename = "isFreeModel", default)]
    pub is_free_model: bool,
}

/// A skill definition sent from frontend
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillDTO {
    pub name: String,
    #[serde(rename = "type")]
    pub skill_type: String, // "prompt" or "tool"
    pub content: Option<String>,          // Markdown (for prompt-type)
    pub tools: Option<serde_json::Value>, // Tool definitions (for tool-type)
    pub execution: Option<serde_json::Value>, // Execution config (for tool-type)
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct TitleGenerationRequest {
    #[validate(length(min = 1, message = "At least one message is required"))]
    #[serde(rename = "lastMessages")]
    pub last_messages: Vec<LastMessageDTO>,
    pub model: Option<String>,
    #[serde(rename = "userLanguage")]
    pub user_language: Option<String>,
}

/// Status of a message persistence confirmation from frontend
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfirmStatus {
    Persisted,
    Failed,
    Unknown,
}

impl From<&str> for ConfirmStatus {
    fn from(s: &str) -> Self {
        match s {
            "persisted" => ConfirmStatus::Persisted,
            "failed" => ConfirmStatus::Failed,
            _ => ConfirmStatus::Unknown,
        }
    }
}

/// Data for a pending credit charge (awaiting confirmation)
#[derive(Clone, Debug)]
pub struct PendingChargeData {
    pub content_hash: String,
    pub openrouter_cost: f64,
    pub model: String,
    pub channel_id: Option<String>,
    pub tokens_prompt: i32,
    pub tokens_completion: i32,
    pub created_at: std::time::Instant,
}

/// Request for intent classification
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IntentClassificationRequest {
    pub prompt: String,
}
