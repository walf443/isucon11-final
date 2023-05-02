use crate::models::class::ClassID;
use crate::models::course::CourseID;

#[derive(Debug, serde::Deserialize)]
pub struct AssignmentPath {
    pub course_id: CourseID,
    pub class_id: ClassID,
}
