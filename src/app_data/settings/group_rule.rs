use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GroupRule {
    #[serde(rename = "id")]
    pub group_name: String,
    pub keywords: Vec<String>,
    #[serde(rename = "max_count")]
    pub max_items: usize,
    #[serde(rename = "priority_bonus")]
    pub priority_bonus: i32,
}
