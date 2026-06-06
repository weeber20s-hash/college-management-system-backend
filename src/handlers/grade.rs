use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::grade::{Grade, GpaSummary, CreateGradeRequest, UpdateGradeRequest, GetGradeQuery};

fn grade_letter_from_percentage(p: f64) -> String {
    if p >= 85.0 { "HD".to_string() } else if p >= 75.0 { "D".to_string() } else if p >= 65.0 { "CR".to_string() } else if p >= 50.0 { "P".to_string() } else { "F".to_string() }
}

pub async fn create_grade(State(state): State<AppState>, Json(payload): Json<CreateGradeRequest>) -> Result<Json<Grade>, (StatusCode, String)> {
    let max_marks: (f64,) = sqlx::query_as("SELECT max_marks FROM assessments WHERE id=$1").bind(payload.assessment_id).fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid assessment: {}", e)))?;
    let percentage = if max_marks.0 > 0.0 { (payload.marks_obtained / max_marks.0) * 100.0 } else { 0.0 };
    let letter = payload.grade_letter.unwrap_or_else(|| grade_letter_from_percentage(percentage));
    let item = sqlx::query_as::<_, Grade>("INSERT INTO grades (student_id,assessment_id,marks_obtained,grade_letter,feedback) VALUES ($1,$2,$3,$4,$5) RETURNING id, student_id, assessment_id, marks_obtained, grade_letter, feedback")
        .bind(payload.student_id).bind(payload.assessment_id).bind(payload.marks_obtained).bind(letter).bind(&payload.feedback)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(item))
}

pub async fn get_grades(State(state): State<AppState>, Query(query): Query<GetGradeQuery>) -> Result<Json<Vec<Grade>>, (StatusCode, String)> {
    let items = if let Some(student_id) = query.student_id {
        sqlx::query_as::<_, Grade>("SELECT g.id, g.student_id, g.assessment_id, g.marks_obtained, g.grade_letter, g.feedback FROM grades g WHERE g.student_id=$1 ORDER BY g.id")
            .bind(student_id).fetch_all(&state.db_pool).await
    } else if let Some(assessment_id) = query.assessment_id {
        sqlx::query_as::<_, Grade>("SELECT id, student_id, assessment_id, marks_obtained, grade_letter, feedback FROM grades WHERE assessment_id=$1 ORDER BY id")
            .bind(assessment_id).fetch_all(&state.db_pool).await
    } else if let Some(course_code) = query.course_code {
        sqlx::query_as::<_, Grade>("SELECT g.id, g.student_id, g.assessment_id, g.marks_obtained, g.grade_letter, g.feedback FROM grades g JOIN assessments a ON a.id=g.assessment_id WHERE a.course_code=$1 ORDER BY g.id")
            .bind(course_code).fetch_all(&state.db_pool).await
    } else {
        sqlx::query_as::<_, Grade>("SELECT id, student_id, assessment_id, marks_obtained, grade_letter, feedback FROM grades ORDER BY id").fetch_all(&state.db_pool).await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(items))
}

pub async fn get_grade_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Grade>, (StatusCode, String)> {
    let item = sqlx::query_as::<_, Grade>("SELECT id, student_id, assessment_id, marks_obtained, grade_letter, feedback FROM grades WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    item.map(Json).ok_or((StatusCode::NOT_FOUND, "Grade not found".to_string()))
}

pub async fn update_grade(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateGradeRequest>) -> Result<Json<Grade>, (StatusCode, String)> {
    let current = sqlx::query_as::<_, Grade>("SELECT id, student_id, assessment_id, marks_obtained, grade_letter, feedback FROM grades WHERE id=$1")
        .bind(id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Grade not found".to_string()))?;
    let updated = sqlx::query_as::<_, Grade>("UPDATE grades SET marks_obtained=$1, grade_letter=$2, feedback=$3 WHERE id=$4 RETURNING id, student_id, assessment_id, marks_obtained, grade_letter, feedback")
        .bind(payload.marks_obtained.unwrap_or(current.marks_obtained)).bind(payload.grade_letter.or(current.grade_letter)).bind(payload.feedback.or(current.feedback)).bind(id)
        .fetch_one(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(updated))
}

pub async fn delete_grade(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM grades WHERE id=$1").bind(id).execute(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if result.rows_affected() == 0 { Err((StatusCode::NOT_FOUND, "Grade not found".to_string())) } else { Ok(StatusCode::NO_CONTENT) }
}

pub async fn get_student_gpa(State(state): State<AppState>, Path(student_id): Path<Uuid>) -> Result<Json<GpaSummary>, (StatusCode, String)> {
    let summary = sqlx::query_as::<_, GpaSummary>(
        "SELECT s.id AS student_id, s.full_name AS student_name, COUNT(g.id)::BIGINT AS total_assessments,
         COALESCE(AVG((g.marks_obtained / NULLIF(a.max_marks,0)) * 100),0)::DOUBLE PRECISION AS average_percentage,
         LEAST(4.0, COALESCE(AVG((g.marks_obtained / NULLIF(a.max_marks,0)) * 100),0) / 25.0)::DOUBLE PRECISION AS gpa
         FROM students s LEFT JOIN grades g ON g.student_id=s.id LEFT JOIN assessments a ON a.id=g.assessment_id
         WHERE s.id=$1 GROUP BY s.id, s.full_name"
    ).bind(student_id).fetch_optional(&state.db_pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    summary.map(Json).ok_or((StatusCode::NOT_FOUND, "Student not found".to_string()))
}
