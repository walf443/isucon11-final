use crate::models::course::CourseID;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Class {
    pub id: ClassID,
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct ClassID(String);

impl ClassID {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Dummy)]
pub struct CreateClass {
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClassWithSubmitted {
    pub id: ClassID,
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
    pub submitted: bool,
}
