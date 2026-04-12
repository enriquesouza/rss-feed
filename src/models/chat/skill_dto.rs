use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillDTO {
    pub name: String,
    #[serde(rename = "type")]
    pub skill_type: String,
    pub content: Option<String>,
    pub tools: Option<serde_json::Value>,
    pub execution: Option<serde_json::Value>,
}
