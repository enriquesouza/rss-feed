use crate::app_data::open_router::ChatMessage;
use crate::app_data::{
    open_router::{ChatCompletionResponse, ChatRequest, UsageConfig},
    settings::app_env::AppEnv,
};
use reqwest::Client;
use serde::Deserialize;
use std::sync::LazyLock;
use std::{fs::read_to_string, path::Path, time::Duration};

const OPEN_ROUTER_TIMEOUT_SECS: u64 = 120;
const OPEN_ROUTER_RETRY_WAIT_SECS: u64 = 2;

pub struct NewsWriter<'a> {
    client: &'a Client,
}
#[derive(Debug, Deserialize)]
pub struct PromptFile {
    #[serde(rename = "system_prompt")]
    prompt: String,
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
        Self { client }
    }

    pub async fn write_news_message(
        &self,
        news_blocks: Vec<String>,
    ) -> anyhow::Result<Option<String>> {
        let news_text = news_blocks.join("\n\n---\n\n");

        let body = ChatRequest {
            model: "x-ai/grok-4.1-fast".into(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: NEWS_MESSAGE_PROMPT.prompt.clone(),
                    tool_calls: None,
                    tool_call_id: None,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: news_text,
                    tool_calls: None,
                    tool_call_id: None,
                },
            ],
            stream: false,
            temperature: Some(0.9),
            max_tokens: Some(4000),
            usage: Some(UsageConfig { include: true }),
            stream_options: None,
            tools: None,
        };

        let response = self.send_chat_request(body).await?;

        let message_text = response
            .choices
            .first()
            .map(|item| item.message.content.clone())
            .unwrap_or_default();

        Ok(Some(message_text))
    }

    pub async fn send_chat_request(
        &self,
        request: ChatRequest,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let api_key = AppEnv::get().open_router_api_key.clone();
        let mut last_problem: Option<anyhow::Error> = None;

        for attempt in 1..=3 {
            let job = async {
                let http_response = self
                    .client
                    .post("https://openrouter.ai/api/v1/chat/completions")
                    .timeout(Duration::from_secs(OPEN_ROUTER_TIMEOUT_SECS))
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                    .header("HTTP-Referer", "https://compra.ai")
                    .header("X-Title", "compra.ai")
                    .json(&request)
                    .send()
                    .await?;
                if !http_response.status().is_success() {
                    let error_text = http_response.text().await?;
                    return Err(anyhow::anyhow!("OpenRouter error: {}", error_text));
                }

                let response: ChatCompletionResponse =
                    http_response.json::<ChatCompletionResponse>().await?;
                Ok(response)
            };

            match job.await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    last_problem = Some(err);
                }
            }

            if attempt < 3 {
                tokio::time::sleep(Duration::from_secs(OPEN_ROUTER_RETRY_WAIT_SECS)).await;
            }
        }

        Err(last_problem.unwrap_or_else(|| anyhow::anyhow!("OpenRouter request failed")))
    }
}
