use isucholar_core::models::day_of_week::DayOfWeek;

#[derive(Debug, serde::Serialize)]
pub struct GetRegisteredCourseResponseContent {
    pub id: String,
    pub name: String,
    pub teacher: String,
    pub period: u8,
    pub day_of_week: DayOfWeek,
}
