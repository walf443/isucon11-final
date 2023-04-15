#[derive(Debug, Default, serde::Serialize)]
pub struct RegisterCoursesErrorResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub course_not_found: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_registrable_status: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schedule_conflict: Vec<String>,
}
