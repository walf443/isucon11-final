use crate::models::course_status::CourseStatus;
use crate::models::course_type::CourseType;
use crate::models::day_of_week::DayOfWeek;
use crate::models::user::UserID;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Course {
    pub id: CourseID,
    pub code: CourseCode,
    #[sqlx(rename = "type")]
    pub type_: CourseType,
    pub name: String,
    pub description: String,
    pub credit: u8,
    pub period: u8,
    pub day_of_week: DayOfWeek,
    pub teacher_id: UserID,
    pub keywords: String,
    pub status: CourseStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct CourseID(String);

impl CourseID {
    pub fn new(course_id: String) -> Self {
        Self(course_id)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct CourseCode(String);

impl CourseCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, serde::Serialize, Dummy)]
pub struct CourseWithTeacher {
    pub id: CourseID,
    pub code: CourseCode,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: String,
    pub name: String,
    pub description: String,
    pub credit: u8,
    pub period: u8,
    pub day_of_week: DayOfWeek,
    #[serde(skip)]
    pub teacher_id: UserID,
    pub keywords: String,
    pub status: CourseStatus,
    pub teacher: String,
}

#[derive(Debug, serde::Deserialize, Dummy)]
pub struct CreateCourse {
    pub id: CourseID,
    pub user_id: UserID,
    pub code: CourseCode,
    #[serde(rename = "type")]
    pub type_: CourseType,
    pub name: String,
    pub description: String,
    pub credit: u8,
    pub period: u8,
    pub day_of_week: DayOfWeek,
    pub keywords: String,
}
