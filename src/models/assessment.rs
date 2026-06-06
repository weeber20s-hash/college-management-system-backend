use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Assessment {
    pub id: Uuid,
    pub course_code: String,
    pub title: String,
    pub assessment_type: String,
    pub max_marks: f64,
    pub weight_percentage: f64,
    pub due_date: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAssessmentRequest {
    pub course_code: String,
    pub title: String,
    pub assessment_type: String,
    pub max_marks: f64,
    pub weight_percentage: f64,
    pub due_date: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssessmentRequest {
    pub course_code: Option<String>,
    pub title: Option<String>,
    pub assessment_type: Option<String>,
    pub max_marks: Option<f64>,
    pub weight_percentage: Option<f64>,
    pub due_date: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetAssessmentQuery {
    pub course_code: Option<String>,
    pub assessment_type: Option<String>,
}
