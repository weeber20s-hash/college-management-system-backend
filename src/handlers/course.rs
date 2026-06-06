use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use crate::config::AppState;
use crate::models::course::{Course, CreateCourseRequest, UpdateCourseRequest, GetCourseQuery};

pub async fn create_course(
    State(state): State<AppState>,
    Json(payload): Json<CreateCourseRequest>,
) -> Result<Json<Course>, (StatusCode, String)> {
    let course = sqlx::query_as::<_, Course>(
        "INSERT INTO courses (course_code, course_name, lecturer, duration, description) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING course_code, course_name, lecturer, duration, description"
    )
    .bind(&payload.course_code)
    .bind(&payload.course_name)
    .bind(&payload.lecturer)
    .bind(&payload.duration)
    .bind(&payload.description)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Create course error: {}", e);
        if e.to_string().contains("unique constraint") {
            (StatusCode::CONFLICT, "Course code already exists".to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;
    
    Ok(Json(course))
}

pub async fn get_courses(
    State(state): State<AppState>,
    Query(query): Query<GetCourseQuery>,
) -> Result<Json<Vec<Course>>, (StatusCode, String)> {
    let courses = if let Some(course_code) = query.course_code {
        sqlx::query_as::<_, Course>(
            "SELECT course_code, course_name, lecturer, duration, description 
             FROM courses WHERE course_code = $1"
        )
        .bind(course_code)
        .fetch_all(&state.db_pool)
        .await
    } else if let Some(course_name) = query.course_name {
        sqlx::query_as::<_, Course>(
            "SELECT course_code, course_name, lecturer, duration, description 
             FROM courses WHERE LOWER(course_name) LIKE LOWER($1)"
        )
        .bind(format!("%{}%", course_name))
        .fetch_all(&state.db_pool)
        .await
    } else if let Some(lecturer) = query.lecturer {

        sqlx::query_as::<_, Course>(
            "SELECT course_code, course_name, lecturer, duration, description 
             FROM courses WHERE LOWER(lecturer) LIKE LOWER($1)"
        )
        .bind(format!("%{}%", lecturer))
        .fetch_all(&state.db_pool)
        .await
    } else {
        sqlx::query_as::<_, Course>(
            "SELECT course_code, course_name, lecturer, duration, description 
             FROM courses ORDER BY course_code"
        )
        .fetch_all(&state.db_pool)
        .await
    };
    
    let courses = courses.map_err(|e| {
        eprintln!("Get courses error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(courses))
}

pub async fn get_course_by_code(
    State(state): State<AppState>,
    Path(course_code): Path<String>,
) -> Result<Json<Course>, (StatusCode, String)> {
    let course = sqlx::query_as::<_, Course>(
        "SELECT course_code, course_name, lecturer, duration, description 
         FROM courses WHERE course_code = $1"
    )
    .bind(&course_code)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get course error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    match course {
        Some(course) => Ok(Json(course)),
        None => Err((StatusCode::NOT_FOUND, "Course not found".to_string())),
    }
}

pub async fn update_course(
    State(state): State<AppState>,
    Path(course_code): Path<String>,
    Json(payload): Json<UpdateCourseRequest>,
) -> Result<Json<Course>, (StatusCode, String)> {

    let existing = sqlx::query!("SELECT course_code FROM courses WHERE course_code = $1", &course_code)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Check course error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if existing.is_none() {
        return Err((StatusCode::NOT_FOUND, "Course not found".to_string()));
    }
    
    let current = sqlx::query_as::<_, Course>(
        "SELECT course_code, course_name, lecturer, duration, description FROM courses WHERE course_code = $1"
    )
    .bind(&course_code)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get current course error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let new_course_name = payload.course_name.unwrap_or(current.course_name);
    let new_lecturer = payload.lecturer.unwrap_or(current.lecturer);
    let new_duration = payload.duration.or(current.duration);
    let new_description = payload.description.or(current.description);

    let updated = sqlx::query_as::<_, Course>(
        "UPDATE courses SET course_name = $1, lecturer = $2, duration = $3, description = $4 
         WHERE course_code = $5 
         RETURNING course_code, course_name, lecturer, duration, description"
    )
    .bind(new_course_name)
    .bind(new_lecturer)
    .bind(new_duration)
    .bind(new_description)
    .bind(&course_code)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Update course error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(updated))
}


pub async fn delete_course(
    State(state): State<AppState>,
    Path(course_code): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!("DELETE FROM courses WHERE course_code = $1", &course_code)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Delete course error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Course not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}