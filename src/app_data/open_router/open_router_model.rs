use serde::{Deserialize, Serialize};

use super::{Architecture, Pricing, TopProvider};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenRouterModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pricing: Option<Pricing>,
    pub context_length: Option<u64>,
    pub architecture: Option<Architecture>,
    pub top_provider: Option<TopProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_parameters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
}
