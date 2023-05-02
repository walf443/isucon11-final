use crate::models::class::ClassID;

#[derive(Debug, serde::Deserialize)]
pub struct AssignmentPath {
    pub course_id: String,
    pub class_id: ClassID,
}
