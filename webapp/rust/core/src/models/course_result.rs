use crate::models::class_score::ClassScore;
use crate::models::course::CourseCode;

#[derive(Debug, serde::Serialize)]
pub struct CourseResult {
    pub name: String,
    pub code: CourseCode,
    pub total_score: i64,
    pub total_score_t_score: f64, // 偏差値
    pub total_score_avg: f64,     // 平均値
    pub total_score_max: i64,     // 最大値
    pub total_score_min: i64,     // 最小値
    pub class_scores: Vec<ClassScore>,
}
