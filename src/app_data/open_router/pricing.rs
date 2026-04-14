use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pricing {
    pub prompt: String,
    pub completion: String,
}
