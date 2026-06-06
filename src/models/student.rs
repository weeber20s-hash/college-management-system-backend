use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: Uuid,
    pub student_id: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub course: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudentRequest {
    pub student_id: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub course: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub course: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetStudentQuery {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub course: Option<String>,
}