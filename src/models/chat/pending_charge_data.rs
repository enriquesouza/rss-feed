#[derive(Clone, Debug)]
pub struct PendingChargeData {
    pub content_hash: String,
    pub openrouter_cost: f64,
    pub model: String,
    pub channel_id: Option<String>,
    pub tokens_prompt: i32,
    pub tokens_completion: i32,
    pub created_at: std::time::Instant,
}
