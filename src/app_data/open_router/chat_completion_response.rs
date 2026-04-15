use serde::{Deserialize, Serialize};

use super::{Choice, Usage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub model: String,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
}
