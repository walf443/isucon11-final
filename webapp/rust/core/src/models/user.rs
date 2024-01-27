use crate::models::user_type::UserType;
use fake::Dummy;
use kubetsu::Id;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, Dummy)]
pub struct User {
    pub id: UserID,
    pub code: UserCode,
    pub name: String,
    pub hashed_password: Vec<u8>,
    #[sqlx(rename = "type")]
    pub type_: UserType,
}

pub type UserID = Id<User, String>;

pub type UserCode = Id<User, UserCodeCode>;

#[derive(Debug, Clone, PartialEq, Dummy, sqlx::Type, Serialize)]
#[sqlx(transparent)]
pub struct UserCodeCode(#[dummy(faker = "6")] String);

impl From<String> for UserCodeCode {
    fn from(value: String) -> Self {
        UserCodeCode(value)
    }
}

impl Display for UserCodeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
