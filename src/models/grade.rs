use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Grade {
    pub id: Uuid,
    pub student_id: Uuid,
    pub assessment_id: Uuid,
    pub marks_obtained: f64,
    pub grade_letter: Option<String>,
    pub feedback: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GpaSummary {
    pub student_id: Uuid,
    pub student_name: String,
    pub total_assessments: i64,
    pub average_percentage: f64,
    pub gpa: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateGradeRequest {
    pub student_id: Uuid,
    pub assessment_id: Uuid,
    pub marks_obtained: f64,
    pub grade_letter: Option<String>,
    pub feedback: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGradeRequest {
    pub marks_obtained: Option<f64>,
    pub grade_letter: Option<String>,
    pub feedback: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetGradeQuery {
    pub student_id: Option<Uuid>,
    pub assessment_id: Option<Uuid>,
    pub course_code: Option<String>,
}
