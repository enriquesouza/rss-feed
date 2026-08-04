use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GroupRule {
    #[serde(rename = "id")]
    pub group_name: String,
    pub keywords: Vec<String>,
    // Broad words that only count when they show up in the title.
    #[serde(default)]
    pub weak_keywords: Vec<String>,
    // Feeds that always talk about this topic (domain or path pieces).
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(rename = "max_count")]
    pub max_items: usize,
    #[serde(rename = "priority_bonus")]
    pub priority_bonus: i32,
}
