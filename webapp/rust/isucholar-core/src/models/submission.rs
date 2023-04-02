#[derive(Debug, sqlx::FromRow)]
pub struct Submission {
    pub user_id: String,
    pub user_code: String,
    pub file_name: String,
}

#[derive(Debug)]
pub struct CreateSubmission {
    pub user_id: String,
    pub class_id: String,
    pub file_name: String,
}
