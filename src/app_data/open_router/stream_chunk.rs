use serde::Deserialize;

use super::{StreamChoice, Usage};

#[derive(Clone, Debug, Deserialize)]
pub struct StreamChunk {
    pub id: Option<String>,
    #[serde(default)]
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
    pub model: Option<String>,
}
