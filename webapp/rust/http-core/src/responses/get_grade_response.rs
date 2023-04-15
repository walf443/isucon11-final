use isucholar_core::models::course_result::CourseResult;
use isucholar_core::models::summary::Summary;

#[derive(Debug, serde::Serialize)]
pub struct GetGradeResponse {
    pub summary: Summary,
    #[serde(rename = "courses")]
    pub course_results: Vec<CourseResult>,
}
