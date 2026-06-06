use axum::{Json, http::StatusCode, extract::{State, Path}};
use uuid::Uuid;
use crate::config::AppState;
use crate::models::notice::{Notice, CreateNoticeRequest, UpdateNoticeRequest};

pub async fn create_notice(
    State(state): State<AppState>,
    Json(payload): Json<CreateNoticeRequest>,
) -> Result<Json<Notice>, (StatusCode, String)> {
    let notice_id = Uuid::new_v4();
    
    let notice = sqlx::query_as::<_, Notice>(
        "INSERT INTO notices (id, title, content) 
         VALUES ($1, $2, $3) 
         RETURNING id, title, content, created_at"
    )
    .bind(notice_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Create notice error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(notice))
}

pub async fn get_all_notices(
    State(state): State<AppState>,
) -> Result<Json<Vec<Notice>>, (StatusCode, String)> {
    let notices = sqlx::query_as::<_, Notice>(
        "SELECT id, title, content, created_at 
         FROM notices 
         ORDER BY created_at DESC"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get notices error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(notices))
}

pub async fn get_notice_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Notice>, (StatusCode, String)> {
    let notice = sqlx::query_as::<_, Notice>(
        "SELECT id, title, content, created_at 
         FROM notices WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get notice error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    match notice {
        Some(notice) => Ok(Json(notice)),
        None => Err((StatusCode::NOT_FOUND, "Notice not found".to_string())),
    }
}

pub async fn update_notice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNoticeRequest>,
) -> Result<Json<Notice>, (StatusCode, String)> {
    let existing = sqlx::query!("SELECT id FROM notices WHERE id = $1", id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Check notice error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if existing.is_none() {
        return Err((StatusCode::NOT_FOUND, "Notice not found".to_string()));
    }
    
    let current = sqlx::query_as::<_, Notice>(
        "SELECT id, title, content, created_at FROM notices WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get current notice error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let new_title = payload.title.unwrap_or(current.title);
    let new_content = payload.content.unwrap_or(current.content);

    let updated = sqlx::query_as::<_, Notice>(
        "UPDATE notices SET title = $1, content = $2 
         WHERE id = $3 
         RETURNING id, title, content, created_at"
    )
    .bind(new_title)
    .bind(new_content)
    .bind(id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Update notice error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(updated))
}

pub async fn delete_notice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!("DELETE FROM notices WHERE id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Delete notice error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Notice not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}