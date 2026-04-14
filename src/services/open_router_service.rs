use crate::models::open_router::ChatMessage;
use crate::models::{
    configs::config::Env,
    open_router::{ChatCompletionResponse, ChatRequest, UsageConfig},
};
use reqwest::Client;
use serde::Deserialize;
use serde_yml;
use std::{fs::read_to_string, path::Path};
pub struct OpenRouterService<'a> {
    client: &'a Client,
}
#[derive(Debug, Deserialize)]
pub struct SystemPrompt {
    system_prompt: String,
}

impl<'a> OpenRouterService<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_optimized_news(&self, request: Vec<String>) -> anyhow::Result<Option<String>> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("prompts")
            .join("news.yml");

        let yml = read_to_string(&path)?;
        let yml_prompt: SystemPrompt = serde_yml::from_str(&yml)?;

        let text = request.join("\n\n---\n\n");

        let chat_request = ChatRequest {
            model: "x-ai/grok-4.1-fast".into(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: yml_prompt.system_prompt,
                    tool_calls: None,
                    tool_call_id: None,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: text,
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

        let completion = self.chat_completion(chat_request).await?;

        let content = completion
            .choices
            .first()
            .map(|item| item.message.content.clone())
            .unwrap_or_default();

        Ok(Some(content))
    }

    pub async fn chat_completion(
        &self,
        request: ChatRequest,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let api_key = Env::new().open_router_api_key.clone();
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 1..=3 {
            let req_future = async {
                let res = self
                    .client
                    .post("https://openrouter.ai/api/v1/chat/completions")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                    .header("HTTP-Referer", "https://compra.ai")
                    .header("X-Title", "compra.ai")
                    .json(&request)
                    .send()
                    .await?;
                if !res.status().is_success() {
                    let error_text = res.text().await?;
                    return Err(anyhow::anyhow!("OpenRouter error: {}", error_text));
                }

                let response: ChatCompletionResponse = res.json::<ChatCompletionResponse>().await?;
                Ok(response)
            };

            match tokio::time::timeout(std::time::Duration::from_secs(120), req_future).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(err)) => {
                    last_error = Some(err);
                }
                Err(_) => {
                    let timeout_error = anyhow::anyhow!("OpenRouter API timeout after 120s");

                    last_error = Some(timeout_error);
                }
            }

            if attempt < 3 {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("OpenRouter request failed")))
    }
}
