#[derive(Debug, sqlx::FromRow)]
pub struct Class {
    pub id: String,
    pub course_id: String,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
}

pub struct CreateClass {
    pub id: String,
    pub course_id: String,
    pub part: u8,
    pub title: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClassWithSubmitted {
    pub id: String,
    pub course_id: String,
    pub part: u8,
    pub title: String,
    pub description: String,
    pub submission_closed: bool,
    pub submitted: bool,
}
