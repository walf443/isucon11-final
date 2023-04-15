use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::day_of_week::DayOfWeek;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct GetCourseDetailResponse {
    pub id: String,
    pub code: String,
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
