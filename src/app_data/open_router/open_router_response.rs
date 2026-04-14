use serde::{Deserialize, Serialize};

use super::OpenRouterModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenRouterResponse {
    pub data: Vec<OpenRouterModel>,
}
