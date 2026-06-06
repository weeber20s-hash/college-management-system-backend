use axum::{Json, http::StatusCode, extract::{State, Path, Query}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::student::{Student, CreateStudentRequest, UpdateStudentRequest, GetStudentQuery};

pub async fn create_student(
    State(state): State<AppState>,
    Json(payload): Json<CreateStudentRequest>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student_id = Uuid::new_v4();
    
    let student = sqlx::query_as::<_, Student>(
        "INSERT INTO students (id, student_id, full_name, email, phone, course, status) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING id, student_id, full_name, email, phone, course, status"
    )
    .bind(student_id)
    .bind(&payload.student_id)
    .bind(&payload.full_name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.course)
    .bind(&payload.status)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Create student error: {}", e);
        if e.to_string().contains("unique constraint") {
            (StatusCode::CONFLICT, "Student ID or Email already exists".to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;
    
    Ok(Json(student))
}

pub async fn get_students(
    State(state): State<AppState>,
    Query(query): Query<GetStudentQuery>,
) -> Result<Json<Vec<Student>>, (StatusCode, String)> {
    let students = if let Some(id) = query.id {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students WHERE id = $1"
        )
        .bind(id)
        .fetch_all(&state.db_pool)
        .await
    } else if let Some(name) = query.name {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students WHERE LOWER(full_name) LIKE LOWER($1)"
        )
        .bind(format!("%{}%", name))
        .fetch_all(&state.db_pool)
        .await
    } else if let Some(course) = query.course {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students WHERE LOWER(course) = LOWER($1)"
        )
        .bind(course)
        .fetch_all(&state.db_pool)
        .await
    } else {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students ORDER BY id"
        )
        .fetch_all(&state.db_pool)
        .await
    };
    
    let students = students.map_err(|e| {
        eprintln!("Get students error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(students))
}

pub async fn get_student_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student = if let Ok(uuid) = Uuid::parse_str(&id) {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students WHERE id = $1"
        )
        .bind(uuid)
        .fetch_optional(&state.db_pool)
        .await
    } else {
        sqlx::query_as::<_, Student>(
            "SELECT id, student_id, full_name, email, phone, course, status 
             FROM students WHERE student_id = $1"
        )
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await
    };
    
    let student = student.map_err(|e| {
        eprintln!("Get student error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    match student {
        Some(student) => Ok(Json(student)),
        None => Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
    }
}

pub async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student_uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => {
            match sqlx::query!("SELECT id FROM students WHERE id = $1", uuid)
                .fetch_optional(&state.db_pool)
                .await
            {
                Ok(Some(record)) => record.id,
                Ok(None) => return Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
                Err(e) => {
                    eprintln!("Check student error: {}", e);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
                }
            }
        }
        Err(_) => {
            match sqlx::query!("SELECT id FROM students WHERE student_id = $1", &id)
                .fetch_optional(&state.db_pool)
                .await
            {
                Ok(Some(record)) => record.id,
                Ok(None) => return Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
                Err(e) => {
                    eprintln!("Check student error: {}", e);
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
                }
            }
        }
    };
    
    let current = sqlx::query_as::<_, Student>(
        "SELECT id, student_id, full_name, email, phone, course, status FROM students WHERE id = $1"
    )
    .bind(student_uuid)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get current student error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let new_full_name = payload.full_name.unwrap_or(current.full_name);
    let new_email = payload.email.unwrap_or(current.email);
    let new_phone = payload.phone.or(current.phone);
    let new_course = payload.course.unwrap_or(current.course);
    let new_status = payload.status.unwrap_or(current.status);
    
    let updated = sqlx::query_as::<_, Student>(
        "UPDATE students SET full_name = $1, email = $2, phone = $3, course = $4, status = $5 
         WHERE id = $6 
         RETURNING id, student_id, full_name, email, phone, course, status"
    )
    .bind(new_full_name)
    .bind(new_email)
    .bind(new_phone)
    .bind(new_course)
    .bind(new_status)
    .bind(student_uuid)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Update student error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(updated))
}

pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = if let Ok(uuid) = Uuid::parse_str(&id) {
        sqlx::query!("DELETE FROM students WHERE id = $1", uuid)
            .execute(&state.db_pool)
            .await
    } else {
        sqlx::query!("DELETE FROM students WHERE student_id = $1", &id)
            .execute(&state.db_pool)
            .await
    };
    
    let result = result.map_err(|e| {
        eprintln!("Delete student error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Student not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}