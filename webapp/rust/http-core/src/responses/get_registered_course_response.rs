use isucholar_core::models::course::CourseID;
use isucholar_core::models::day_of_week::DayOfWeek;

#[derive(Debug, serde::Serialize)]
pub struct GetRegisteredCourseResponseContent {
    pub id: CourseID,
    pub name: String,
    pub teacher: String,
    pub period: u8,
    pub day_of_week: DayOfWeek,
}
