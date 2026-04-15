use crate::app_data::open_router::ChatMessage;
use crate::app_data::open_router::ProviderPreferences;
use crate::app_data::open_router::chat_message::MessageContent;
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

    pub async fn write_news_message(&self, news_text: String) -> anyhow::Result<Option<String>> {
        let body = ChatRequest {
            model: "x-ai/grok-4.1-fast".into(),
            messages: vec![
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
            ],
            stream: false,
            temperature: Some(0.3),
            max_tokens: Some(4000),
            max_completion_tokens: None,
            usage: Some(UsageConfig { include: true }),
            stream_options: None,
            tools: None,
            models: None,
            provider: Some(ProviderPreferences {
                data_collection: Some("deny".to_string()),
                ..Default::default()
            }),
            response_format: None,
            stop: None,
            tool_choice: None,
            parallel_tool_calls: None,
            plugins: None,
            reasoning: Some(serde_json::json!({
                "effort": "high"
            })),
            user: None,
            route: None,
            top_p: None,
            top_k: None,
            frequency_penalty: None,
            presence_penalty: None,
            repetition_penalty: None,
            min_p: None,
            top_a: None,
            seed: None,
        };

        let response = self.send_chat_request(body).await?;

        let message_text = response
            .choices
            .first()
            .and_then(|item| item.message.text_content())
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
                    .header("X-OpenRouter-Title", "compra.ai")
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
