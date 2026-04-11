use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenRouterResponse {
    pub data: Vec<OpenRouterModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenRouterModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pricing: Option<Pricing>,
    pub context_length: Option<u64>,
    pub architecture: Option<Architecture>,
    pub top_provider: Option<TopProvider>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pricing {
    pub prompt: String,
    pub completion: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Architecture {
    pub modality: String,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopProvider {
    pub context_length: Option<u64>,
    pub max_completion_tokens: Option<u64>,
    pub is_moderated: Option<bool>,
}

// ============ Tool / Function Calling Types ============

/// Function parameter definition (JSON Schema)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema object
}

/// A tool definition sent to OpenRouter (OpenAI-compatible)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String, // "function"
    pub function: FunctionDefinition,
}

/// A tool call returned by the LLM in a response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String, // "function"
    pub function: FunctionCallData,
}

/// The function name + arguments the LLM wants to call
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCallData {
    pub name: String,
    pub arguments: String, // JSON string of arguments
}

/// Partial tool call for streaming (delta accumulation)
#[derive(Clone, Debug, Deserialize)]
pub struct ToolCallDelta {
    pub index: Option<i32>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub call_type: Option<String>,
    pub function: Option<FunctionCallDeltaData>,
}

/// Partial function data in streaming
#[derive(Clone, Debug, Deserialize)]
pub struct FunctionCallDeltaData {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

// ============ Chat Message (supports tool roles) ============

/// Chat message — supports regular roles + tool results
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    /// Tool calls requested by assistant (only present when role=assistant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Links a tool result to the original tool_call (only present when role=tool)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Configuration to request usage data in streaming responses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageConfig {
    pub include: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ChatRequest {
    #[validate(length(min = 1, message = "Model ID is required"))]
    pub model: String,
    #[validate(length(min = 1, message = "At least one message is required"))]
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0.0, max = 2.0, message = "Temperature must be between 0 and 2"))]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1, message = "Max tokens must be at least 1"))]
    pub max_tokens: Option<i32>,
    /// Request usage data in streaming response (set to { include: true })
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageConfig>,
    /// Stream options (Standard OpenAI format to include usage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    /// Tool definitions for function calling (OpenAI-compatible)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub model: String,
    pub usage: Option<Usage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

/// Usage data returned by OpenRouter (token counts and cost)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: Option<i32>,
    /// Total cost in USD (only present if usage.include = true)
    #[serde(default)]
    pub cost: Option<f64>,
    /// Fallback for pre-paid BYOK accounts where primary cost is 0
    pub cost_details: Option<CostDetails>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostDetails {
    pub upstream_inference_cost: Option<f64>,
    pub upstream_inference_prompt_cost: Option<f64>,
    pub upstream_inference_completions_cost: Option<f64>,
}

/// SSE streaming chunk structure
#[derive(Clone, Debug, Deserialize)]
pub struct StreamChunk {
    pub id: Option<String>,
    #[serde(default)]
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
    pub model: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: Option<Delta>,
    pub finish_reason: Option<String>,
    #[serde(default)]
    pub index: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
    /// Tool calls requested by the LLM during streaming
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}
