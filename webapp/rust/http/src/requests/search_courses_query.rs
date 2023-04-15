use isucholar_core::models::day_of_week::DayOfWeek;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SearchCoursesQuery {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub credit: Option<i64>,
    pub teacher: Option<String>,
    pub period: Option<i64>,
    pub day_of_week: Option<DayOfWeek>,
    pub keywords: Option<String>,
    pub status: Option<String>,
    pub page: Option<String>,
}
