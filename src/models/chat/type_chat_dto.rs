use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TypeChatDTO {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub prefix: Option<String>,
}
