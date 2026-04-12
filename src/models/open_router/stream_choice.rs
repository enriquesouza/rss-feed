use serde::Deserialize;

use super::Delta;

#[derive(Clone, Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: Option<Delta>,
    pub finish_reason: Option<String>,
    #[serde(default)]
    pub index: i32,
}
