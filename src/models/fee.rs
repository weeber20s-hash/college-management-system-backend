use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Fee {
    pub id: Uuid,
    pub student_id: Uuid,
    pub fee_type: String,
    pub amount: f64,
    pub paid_amount: f64,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub payment_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FeeSummary {
    pub student_id: Uuid,
    pub student_name: String,
    pub total_fees: f64,
    pub total_paid: f64,
    pub outstanding_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateFeeRequest {
    pub student_id: Uuid,
    pub fee_type: String,
    pub amount: f64,
    pub paid_amount: Option<f64>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub payment_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeeRequest {
    pub fee_type: Option<String>,
    pub amount: Option<f64>,
    pub paid_amount: Option<f64>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub payment_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFeeQuery {
    pub student_id: Option<Uuid>,
    pub status: Option<String>,
    pub fee_type: Option<String>,
}
