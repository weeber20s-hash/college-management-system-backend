use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize)]
pub struct StudentReport {
    pub total_students: i64,
    pub active_students: i64,
    pub inactive_students: i64,
    pub graduated_students: i64,
    pub students_by_course: Vec<CourseStat>,
}

#[derive(Debug, Serialize)]
pub struct CourseStat {
    pub course_name: String,
    pub student_count: i64,
}

#[derive(Debug, Serialize)]
pub struct AttendanceReport {
    pub overall_attendance_rate: f64,
    pub total_classes: i64,
    pub total_present: i64,
    pub total_absent: i64,
    pub attendance_by_course: Vec<CourseAttendanceStat>,
    pub low_attendance_students: Vec<LowAttendanceStudent>,
}

#[derive(Debug, Serialize)]
pub struct CourseAttendanceStat {
    pub course_code: String,
    pub course_name: String,
    pub attendance_rate: f64,
    pub total_classes: i64,
}

#[derive(Debug, Serialize)]
pub struct LowAttendanceStudent {
    pub student_id: String,
    pub full_name: String,
    pub course: String,
    pub attendance_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct CourseReport {
    pub course_code: String,
    pub course_name: String,
    pub lecturer: String,
    pub total_students: i64,
    pub attendance_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct DateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}