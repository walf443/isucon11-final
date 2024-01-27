use crate::models::course::CourseID;
use fake::Dummy;
use kubetsu::Id;

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Class {
    pub id: ClassID,
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
}

pub type ClassID = Id<Class, String>;

#[derive(Dummy)]
pub struct CreateClass {
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClassWithSubmitted {
    pub id: ClassID,
    pub course_id: CourseID,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
    pub submitted: bool,
}
