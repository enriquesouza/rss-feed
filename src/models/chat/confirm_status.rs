#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfirmStatus {
    Persisted,
    Failed,
    Unknown,
}

impl From<&str> for ConfirmStatus {
    fn from(s: &str) -> Self {
        match s {
            "persisted" => ConfirmStatus::Persisted,
            "failed" => ConfirmStatus::Failed,
            _ => ConfirmStatus::Unknown,
        }
    }
}
