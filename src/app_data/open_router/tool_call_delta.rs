use serde::Deserialize;

use super::FunctionCallDeltaData;

#[derive(Clone, Debug, Deserialize)]
pub struct ToolCallDelta {
    pub index: Option<i32>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub call_type: Option<String>,
    pub function: Option<FunctionCallDeltaData>,
}
