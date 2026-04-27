use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct ResponseParameters {
    pub retry_after: Option<u32>,
    pub migrate_to_chat_id: Option<i64>,
}