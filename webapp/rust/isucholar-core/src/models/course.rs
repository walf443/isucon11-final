use crate::models::course_status::CourseStatus;
use crate::models::course_type::CourseType;
use crate::models::day_of_week::DayOfWeek;

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
    pub id: String,
    pub code: String,
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