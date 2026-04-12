use serde::{Deserialize, Serialize};

use super::{Choice, Usage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub model: String,
    pub usage: Option<Usage>,
}
