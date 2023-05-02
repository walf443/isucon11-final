use crate::models::user_type::UserType;
use fake::{Dummy, Fake};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct User {
    pub id: String,
    #[dummy(faker = "6")]
    pub code: String,
    pub name: String,
    pub hashed_password: Vec<u8>,
    #[sqlx(rename = "type")]
    pub type_: UserType,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserID(String);

impl UserID {
    pub fn new(user_id: String) -> Self {
        Self(user_id)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
