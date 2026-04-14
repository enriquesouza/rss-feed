use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}
