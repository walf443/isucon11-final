use crate::models::user_type::UserType;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub code: String,
    pub name: String,
    pub hashed_password: Vec<u8>,
    #[sqlx(rename = "type")]
    pub type_: UserType,
}

