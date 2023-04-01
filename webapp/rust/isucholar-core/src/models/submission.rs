#[derive(Debug, sqlx::FromRow)]
pub struct Submission {
    pub user_id: String,
    pub user_code: String,
    pub file_name: String,
}
