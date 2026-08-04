use crate::app_data::{
    omlx::{NEWS_WRITER_MODEL, OmlxClient},
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

pub struct NewsWriter<'a> {
    omlx: OmlxClient<'a>,
}

pub static NEWS_MESSAGE_PROMPT: LazyLock<PromptFile> = LazyLock::new(|| {
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("prompts")
        .join("news_message.yml");

    let file_text =
        read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not read {:?}", file_path));

    serde_norway::from_str(&file_text).expect("Failed to parse news_message.yml")
});

impl<'a> NewsWriter<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        let base_url = AppEnv::get().omlx_host.clone();
        let api_key = AppEnv::get().omlx_api_key.clone();
        Self {
            omlx: OmlxClient::new(client, base_url, api_key),
        }
    }

    pub async fn write_news_message(&self, news_text: String) -> anyhow::Result<Option<String>> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(NEWS_MESSAGE_PROMPT.prompt.clone())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(news_text)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ];

        let response = self
            .omlx
            .chat_completion(NEWS_WRITER_MODEL, messages, 0.3, 4000, Some("max"))
            .await?;

        let message_text = response
            .choices
            .first()
            .and_then(|item| item.message.text_content())
            .unwrap_or_default();

        if message_text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(message_text))
        }
    }
}
