use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCallData {
    pub name: String,
    pub arguments: String,
}
