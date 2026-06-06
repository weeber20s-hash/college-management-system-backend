use serde::{Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub student_count: i64,
    pub active_courses_count: i64,
    pub average_attendance_rate: f64,
    pub total_attendance_records: i64,
}

#[derive(Debug, Serialize)]
pub struct WeeklyAttendance {
    pub week_start: NaiveDate,
    pub week_end: NaiveDate,
    pub attendance_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct SmartAlert {
    pub alert_type: String,
    pub title: String,
    pub message: String,
    pub priority: u8, 
}

#[derive(Debug, Serialize)]
pub struct UpcomingTask {
    pub header: String,
    pub content: String,
    pub due_date: Option<NaiveDate>,
}