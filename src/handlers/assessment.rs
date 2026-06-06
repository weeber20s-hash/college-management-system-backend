use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::assessment::{Assessment, CreateAssessmentRequest, UpdateAssessmentRequest, GetAssessmentQuery};

pub async fn create_assessment(State(state): State<AppState>, Json(payload): Json<CreateAssessmentRequest>) -> Result<Json<Assessment>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Assessment>("INSERT INTO assessments (course_code,title,assessment_type,max_marks,weight_percentage,due_date,description) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description")
        .bind(&payload.course_code).bind(&payload.title).bind(&payload.assessment_type).bind(payload.max_marks).bind(payload.weight_percentage).bind(payload.due_date).bind(&payload.description)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn get_assessments(State(state): State<AppState>, Query(query): Query<GetAssessmentQuery>) -> Result<Json<Vec<Assessment>>, (StatusCode, String)> {
    let items = if let Some(course_code) = query.course_code {
        sqlx::query_as::<_, Assessment>("SELECT id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description FROM assessments WHERE course_code=$1 ORDER BY due_date NULLS LAST")
            .bind(course_code).fetch_all(&state.db_pool).await
    } else if let Some(kind) = query.assessment_type {
        sqlx::query_as::<_, Assessment>("SELECT id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description FROM assessments WHERE LOWER(assessment_type)=LOWER($1) ORDER BY due_date NULLS LAST")
            .bind(kind).fetch_all(&state.db_pool).await
    } else {
        sqlx::query_as::<_, Assessment>("SELECT id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description FROM assessments ORDER BY due_date NULLS LAST")
            .fetch_all(&state.db_pool).await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_assessment_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Assessment>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Assessment>("SELECT id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description FROM assessments WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    item.map(Json).ok_or((StatusCode::NOT_FOUND, "Assessment not found".to_string()))
}

pub async fn update_assessment(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateAssessmentRequest>) -> Result<Json<Assessment>, (StatusCode, String)> {
    let current = sqlx::query_as::<_, Assessment>("SELECT id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description FROM assessments WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Assessment not found".to_string()))?;
    let updated = sqlx::query_as::<_, Assessment>("UPDATE assessments SET course_code=$1,title=$2,assessment_type=$3,max_marks=$4,weight_percentage=$5,due_date=$6,description=$7 WHERE id=$8 RETURNING id, course_code, title, assessment_type, max_marks, weight_percentage, due_date, description")
        .bind(payload.course_code.unwrap_or(current.course_code)).bind(payload.title.unwrap_or(current.title)).bind(payload.assessment_type.unwrap_or(current.assessment_type))
        .bind(payload.max_marks.unwrap_or(current.max_marks)).bind(payload.weight_percentage.unwrap_or(current.weight_percentage)).bind(payload.due_date.or(current.due_date)).bind(payload.description.or(current.description)).bind(id)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_assessment(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM assessments WHERE id=$1").bind(id).execute(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if result.rows_affected() == 0 { Err((StatusCode::NOT_FOUND, "Assessment not found".to_string())) } else { Ok(StatusCode::NO_CONTENT) }
}
