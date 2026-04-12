use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IntentClassificationRequest {
    pub prompt: String,
}
