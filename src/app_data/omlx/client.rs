use crate::app_data::open_router::{ChatCompletionResponse, ChatMessage};
use reqwest::Client;
use serde_json::Value;

/// oMLX client (Apple MLX server) via OpenAI-compatible API
#[derive(Clone, Debug)]
pub struct OmlxClient<'a> {
    http_client: &'a Client,
    base_url: String,
    api_key: Option<String>,
}

impl<'a> OmlxClient<'a> {
    /// Create a new oMLX client
    pub fn new(http_client: &'a Client, base_url: String, api_key: Option<String>) -> Self {
        Self {
            http_client,
            base_url,
            api_key,
        }
    }

    /// Send a chat completion request to oMLX
    ///
    /// `reasoning_effort` is optional and passed as-is to the API (e.g. "low", "high", "none").
    /// Supported by reasoning-capable models like DeepSeek-V4.
    pub async fn chat_completion(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        temperature: f32,
        max_tokens: i32,
        reasoning_effort: Option<&str>,
    ) -> anyhow::Result<ChatCompletionResponse> {
        const BASE_PATH: &str = "/v1/chat/completions";
        const MAX_RETRIES: usize = 3;
        const BACKOFF_SECONDS: u64 = 2;

        for attempt in 0..MAX_RETRIES {
            let mut body = serde_json::json!({
                "model": model,
                "messages": messages,
                "temperature": temperature,
                "max_tokens": max_tokens,
                "stream":false,
                "think": true
            });

            if let Some(effort) = reasoning_effort {
                body["reasoning_effort"] = Value::String(effort.to_string());
            }

            let mut request = self
                .http_client
                .post(format!("{}{}", self.base_url, BASE_PATH));
            if let Some(key) = &self.api_key {
                request = request.bearer_auth(key);
            }
            let response = match request
                .json(&body)
                .timeout(std::time::Duration::from_secs(60 * 10))
                .send()
                .await
            {
                Ok(response) => response,
                Err(e) => {
                    if attempt == MAX_RETRIES - 1 {
                        anyhow::bail!("Failed to send request: {}", e);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(BACKOFF_SECONDS)).await;
                    continue;
                }
            };

            if response.status().is_success() {
                let result: ChatCompletionResponse = response
                    .json()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;
                return Ok(result);
            }

            if attempt < MAX_RETRIES - 1 {
                tokio::time::sleep(tokio::time::Duration::from_secs(BACKOFF_SECONDS)).await;
            }
        }

        anyhow::bail!("All retry attempts failed");
    }
}
