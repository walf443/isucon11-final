use crate::models::user_type::UserType;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
}

impl Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize)]
#[sqlx(transparent)]
pub struct UserCode(#[dummy(faker = "6")] String);

impl UserCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }
}

impl Display for UserCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
