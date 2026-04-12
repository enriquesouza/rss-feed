use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Architecture {
    pub modality: String,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}
