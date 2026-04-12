use serde::{Deserialize, Serialize};

use super::FunctionDefinition;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}
