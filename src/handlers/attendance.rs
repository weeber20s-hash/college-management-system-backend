use axum::{Json, http::StatusCode, extract::{State, Query, Path}};
use uuid::Uuid;
use chrono::NaiveDate;
use sqlx::QueryBuilder;
use crate::config::AppState;
use crate::models::attendance::{Attendance, MarkAttendanceRequest, GetAttendanceQuery, AttendanceSummary};

pub async fn mark_attendance(
    State(state): State<AppState>,
    Json(payload): Json<MarkAttendanceRequest>,
) -> Result<Json<Attendance>, (StatusCode, String)> {

    let student = sqlx::query!("SELECT id FROM students WHERE id = $1", payload.student_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Check student error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if student.is_none() {
        return Err((StatusCode::NOT_FOUND, "Student not found".to_string()));
    }
    

    let course = sqlx::query!("SELECT course_code FROM courses WHERE course_code = $1", &payload.course_code)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Check course error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if course.is_none() {
        return Err((StatusCode::NOT_FOUND, "Course not found".to_string()));
    }
    
    let existing = sqlx::query!(
        "SELECT id FROM attendance WHERE student_id = $1 AND course_code = $2 AND date = $3",
        payload.student_id,
        payload.course_code,
        payload.date
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Check existing attendance error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    if existing.is_some() {
        let attendance = sqlx::query_as::<_, Attendance>(
            "UPDATE attendance SET status = $1 
             WHERE student_id = $2 AND course_code = $3 AND date = $4
             RETURNING id, student_id, course_code, date, status"
        )
        .bind(payload.status)
        .bind(payload.student_id)
        .bind(&payload.course_code)
        .bind(payload.date)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Update attendance error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        
        Ok(Json(attendance))
    } else {
        let attendance = sqlx::query_as::<_, Attendance>(
            "INSERT INTO attendance (id, student_id, course_code, date, status) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, student_id, course_code, date, status"
        )
        .bind(Uuid::new_v4())
        .bind(payload.student_id)
        .bind(&payload.course_code)
        .bind(payload.date)
        .bind(payload.status)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Create attendance error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        
        Ok(Json(attendance))
    }
}

pub async fn get_attendance(
    State(state): State<AppState>,
    Query(query): Query<GetAttendanceQuery>,
) -> Result<Json<Vec<Attendance>>, (StatusCode, String)> {
    let mut query_builder = QueryBuilder::new(
        "SELECT id, student_id, course_code, date, status FROM attendance WHERE 1=1"
    );
    
    if let Some(student_id) = query.student_id {
        query_builder.push(" AND student_id = ");
        query_builder.push_bind(student_id);
    }
    
    if let Some(course_code) = query.course_code {
        query_builder.push(" AND course_code = ");
        query_builder.push_bind(course_code);
    }
    
    if let Some(start_date) = query.start_date {
        query_builder.push(" AND date >= ");
        query_builder.push_bind(start_date);
    }
    
    if let Some(end_date) = query.end_date {
        query_builder.push(" AND date <= ");
        query_builder.push_bind(end_date);
    }
    
    query_builder.push(" ORDER BY date DESC");
    
    let attendance = query_builder
        .build_query_as::<Attendance>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get attendance error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    Ok(Json(attendance))
}

pub async fn get_attendance_summary(
    State(state): State<AppState>,
    Path((student_id, course_code)): Path<(Uuid, String)>,
) -> Result<Json<AttendanceSummary>, (StatusCode, String)> {
    let student = sqlx::query!("SELECT full_name FROM students WHERE id = $1", student_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Get student error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let student_name = match student {
        Some(s) => s.full_name,
        None => return Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
    };

    let stats = sqlx::query!(
        "SELECT 
            COUNT(*) as total_classes,
            SUM(CASE WHEN status = true THEN 1 ELSE 0 END) as present_count
         FROM attendance 
         WHERE student_id = $1 AND course_code = $2",
        student_id,
        course_code
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get attendance stats error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let total_classes = stats.total_classes.unwrap_or(0);
    let present_count = stats.present_count.unwrap_or(0);
    let attendance_percentage = if total_classes > 0 {
        (present_count as f64 / total_classes as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(Json(AttendanceSummary {
        student_id,
        student_name,
        course_code,
        total_classes,
        present_count,
        attendance_percentage,
    }))
}

pub async fn get_attendance_by_date(
    State(state): State<AppState>,
    Path(date): Path<String>,
) -> Result<Json<Vec<Attendance>>, (StatusCode, String)> {
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid date format. Use YYYY-MM-DD".to_string()))?;
    
    let attendance = sqlx::query_as::<_, Attendance>(
        "SELECT id, student_id, course_code, date, status 
         FROM attendance WHERE date = $1 
         ORDER BY course_code, student_id"
    )
    .bind(parsed_date)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get attendance by date error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(attendance))
}