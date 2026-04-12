use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LastMessageDTO {
    pub role: String,
    pub content: String,
}
