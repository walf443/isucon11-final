use crate::models::class::ClassID;

#[derive(Debug, serde::Serialize)]
pub struct ClassScore {
    pub class_id: ClassID,
    pub title: String,
    pub part: u8,
    pub score: Option<i64>, // 0~100点
    pub submitters: i64,    // 提出した学生数
}
