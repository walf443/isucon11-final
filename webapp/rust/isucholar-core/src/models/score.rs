#[derive(Debug, serde::Deserialize)]
pub struct Score {
    pub user_code: String,
    pub score: i64,
}
