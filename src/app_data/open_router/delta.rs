use serde::Deserialize;

use super::ToolCallDelta;

#[derive(Clone, Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}
