#[derive(Debug, Default, serde::Serialize)]
pub struct Summary {
    pub credits: i64,
    pub gpa: f64,
    pub gpa_t_score: f64, // 偏差値
    pub gpa_avg: f64,     // 平均値
    pub gpa_max: f64,     // 最大値
    pub gpa_min: f64,     // 最小値
}
