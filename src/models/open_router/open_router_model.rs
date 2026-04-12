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
}
