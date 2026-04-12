use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostDetails {
    pub upstream_inference_cost: Option<f64>,
    pub upstream_inference_prompt_cost: Option<f64>,
    pub upstream_inference_completions_cost: Option<f64>,
}
