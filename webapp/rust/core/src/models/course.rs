use crate::models::course_status::CourseStatus;
use crate::models::course_type::CourseType;
use crate::models::day_of_week::DayOfWeek;
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
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
    pub teacher_id: String,
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

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct CourseWithTeacher {
    pub id: String,
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
    pub teacher_id: String,
    pub keywords: String,
    pub status: CourseStatus,
    pub teacher: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateCourse {
    pub id: String,
    pub user_id: String,
    pub code: CourseCode,
    #[serde(rename = "type")]
    pub type_: CourseType,
    pub name: String,
    pub description: String,
    pub credit: i64,
    pub period: i64,
    pub day_of_week: DayOfWeek,
    pub keywords: String,
}
