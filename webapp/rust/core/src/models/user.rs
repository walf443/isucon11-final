use crate::models::user_type::UserType;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct User {
    pub id: UserID,
    pub code: UserCode,
    pub name: String,
    pub hashed_password: Vec<u8>,
    #[sqlx(rename = "type")]
    pub type_: UserType,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize)]
#[sqlx(transparent)]
pub struct UserCode(#[dummy(faker = "6")] String);

impl UserCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
