use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub course_code: String,
    pub course_name: String,
    pub lecturer: String,
    pub duration: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub course_code: String,
    pub course_name: String,
    pub lecturer: String,
    pub duration: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub course_name: Option<String>,
    pub lecturer: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetCourseQuery {
    pub course_code: Option<String>,
    pub course_name: Option<String>,
    pub lecturer: Option<String>,
}