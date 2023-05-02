use crate::models::class::ClassID;
use crate::models::user::UserCode;
use fake::{Dummy, Fake};

#[derive(Debug, Dummy)]
pub struct CreateSubmission {
    pub user_id: String,
    pub class_id: ClassID,
    pub file_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SubmissionWithUserCode {
    pub user_id: String,
    pub user_code: UserCode,
    pub file_name: String,
}
