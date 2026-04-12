use serde::{Deserialize, Serialize};

use super::CostDetails;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: Option<i32>,
    #[serde(default)]
    pub cost: Option<f64>,
    pub cost_details: Option<CostDetails>,
}
