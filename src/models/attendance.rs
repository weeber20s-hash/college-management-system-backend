use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attendance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_code: String,
    pub date: NaiveDate,
    pub status: bool, 
}

#[derive(Debug, Deserialize)]
pub struct MarkAttendanceRequest {
    pub student_id: Uuid,
    pub course_code: String,
    pub date: NaiveDate,
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetAttendanceQuery {
    pub student_id: Option<Uuid>,
    pub course_code: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct AttendanceSummary {
    pub student_id: Uuid,
    pub student_name: String,
    pub course_code: String,
    pub total_classes: i64,
    pub present_count: i64,
    pub attendance_percentage: f64,
}