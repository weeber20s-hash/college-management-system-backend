use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::fee::{Fee, FeeSummary, CreateFeeRequest, UpdateFeeRequest, GetFeeQuery};

pub async fn create_fee(State(state): State<AppState>, Json(payload): Json<CreateFeeRequest>) -> Result<Json<Fee>, (StatusCode, String)> {
    let paid = payload.paid_amount.unwrap_or(0.0);
    let status = payload.status.unwrap_or_else(|| if paid >= payload.amount { "Paid".to_string() } else if paid > 0.0 { "Partial".to_string() } else { "Unpaid".to_string() });
    let item = sqlx::query_as::<_, Fee>("INSERT INTO fees (student_id,fee_type,amount,paid_amount,due_date,status,payment_date,notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes")
        .bind(payload.student_id).bind(&payload.fee_type).bind(payload.amount).bind(paid).bind(payload.due_date).bind(status).bind(payload.payment_date).bind(&payload.notes)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn get_fees(State(state): State<AppState>, Query(query): Query<GetFeeQuery>) -> Result<Json<Vec<Fee>>, (StatusCode, String)> {
    let items = if let Some(student_id) = query.student_id {
        sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees WHERE student_id=$1 ORDER BY due_date NULLS LAST")
            .bind(student_id).fetch_all(&state.db_pool).await
    } else if let Some(status) = query.status {
        sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees WHERE LOWER(status)=LOWER($1) ORDER BY due_date NULLS LAST")
            .bind(status).fetch_all(&state.db_pool).await
    } else if let Some(fee_type) = query.fee_type {
        sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees WHERE LOWER(fee_type) LIKE LOWER($1) ORDER BY due_date NULLS LAST")
            .bind(format!("%{}%", fee_type)).fetch_all(&state.db_pool).await
    } else {
        sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees ORDER BY due_date NULLS LAST").fetch_all(&state.db_pool).await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_fee_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Fee>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    item.map(Json).ok_or((StatusCode::NOT_FOUND, "Fee not found".to_string()))
}

pub async fn update_fee(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateFeeRequest>) -> Result<Json<Fee>, (StatusCode, String)> {
    let current = sqlx::query_as::<_, Fee>("SELECT id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes FROM fees WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee not found".to_string()))?;
    let amount = payload.amount.unwrap_or(current.amount);
    let paid = payload.paid_amount.unwrap_or(current.paid_amount);
    let status = payload.status.unwrap_or_else(|| if paid >= amount { "Paid".to_string() } else if paid > 0.0 { "Partial".to_string() } else { "Unpaid".to_string() });
    let updated = sqlx::query_as::<_, Fee>("UPDATE fees SET fee_type=$1, amount=$2, paid_amount=$3, due_date=$4, status=$5, payment_date=$6, notes=$7 WHERE id=$8 RETURNING id, student_id, fee_type, amount, paid_amount, due_date, status, payment_date, notes")
        .bind(payload.fee_type.unwrap_or(current.fee_type)).bind(amount).bind(paid).bind(payload.due_date.or(current.due_date)).bind(status).bind(payload.payment_date.or(current.payment_date)).bind(payload.notes.or(current.notes)).bind(id)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_fee(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM fees WHERE id=$1").bind(id).execute(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if result.rows_affected() == 0 { Err((StatusCode::NOT_FOUND, "Fee not found".to_string())) } else { Ok(StatusCode::NO_CONTENT) }
}

pub async fn get_fee_summary(State(state): State<AppState>, Path(student_id): Path<Uuid>) -> Result<Json<FeeSummary>, (StatusCode, String)> {
    let summary = sqlx::query_as::<_, FeeSummary>(
        "SELECT s.id AS student_id, s.full_name AS student_name,
         COALESCE(SUM(f.amount),0)::DOUBLE PRECISION AS total_fees,
         COALESCE(SUM(f.paid_amount),0)::DOUBLE PRECISION AS total_paid,
         (COALESCE(SUM(f.amount),0) - COALESCE(SUM(f.paid_amount),0))::DOUBLE PRECISION AS outstanding_balance
         FROM students s LEFT JOIN fees f ON f.student_id=s.id WHERE s.id=$1 GROUP BY s.id, s.full_name"
    ).bind(student_id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    summary.map(Json).ok_or((StatusCode::NOT_FOUND, "Student not found".to_string()))
}
