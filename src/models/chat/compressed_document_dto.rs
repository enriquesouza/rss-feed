use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompressedDocumentDTO {
    pub id: i64,
    pub content: String,
}
