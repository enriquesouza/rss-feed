use crate::app_data::{
    omlx::{OmlxClient, X_POSTS_MODEL},
    open_router::ChatMessage,
    open_router::chat_message::MessageContent,
    settings::app_env::AppEnv,
};
use serde::Deserialize;
use std::sync::LazyLock;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Deserialize)]
pub struct PromptFile {
    #[serde(rename = "system_prompt")]
    prompt: String,
}

/// Writes ready-to-review X (Twitter) post drafts for Enrique and Aizzy
/// from the day's top story groups. The human always reviews and posts.
pub struct XPostsWriter<'a> {
    omlx: OmlxClient<'a>,
}

pub static X_POSTS_PROMPT: LazyLock<PromptFile> = LazyLock::new(|| {
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("prompts")
        .join("x_posts.yml");

    let file_text =
        read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not read {:?}", file_path));

    serde_norway::from_str(&file_text).expect("Failed to parse x_posts.yml")
});

impl<'a> XPostsWriter<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        let base_url = AppEnv::get().omlx_host.clone();
        let api_key = AppEnv::get().omlx_api_key.clone();
        Self {
            omlx: OmlxClient::new(client, base_url, api_key),
        }
    }

    pub async fn write_x_posts(&self, day_stories_text: String) -> anyhow::Result<Option<String>> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(X_POSTS_PROMPT.prompt.clone())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(day_stories_text)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let response = self
            .omlx
            .chat_completion(X_POSTS_MODEL, messages, 0.4, 2000, Some("max"))
            .await?;

        let posts_text = response
            .choices
            .first()
            .and_then(|item| item.message.text_content())
            .unwrap_or_default();

        if posts_text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(posts_text))
        }
    }
}
