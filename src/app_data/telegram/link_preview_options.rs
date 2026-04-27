use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
}