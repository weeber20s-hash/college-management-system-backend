use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timetable {
    pub id: Uuid,
    pub course_code: String,
    pub day_of_week: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: Option<String>,
    pub lecturer: Option<String>,
    pub effective_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTimetableRequest {
    pub course_code: String,
    pub day_of_week: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: Option<String>,
    pub lecturer: Option<String>,
    pub effective_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimetableRequest {
    pub course_code: Option<String>,
    pub day_of_week: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub room: Option<String>,
    pub lecturer: Option<String>,
    pub effective_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct GetTimetableQuery {
    pub course_code: Option<String>,
    pub day_of_week: Option<String>,
    pub lecturer: Option<String>,
}
