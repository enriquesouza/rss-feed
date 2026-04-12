use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionCallDeltaData {
    pub name: Option<String>,
    pub arguments: Option<String>,
}
