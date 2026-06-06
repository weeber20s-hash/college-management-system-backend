use axum::{Json, http::StatusCode, extract::State};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::models::admin::{LoginRequest, LoginResponse, CreateAdminRequest};
use crate::config::AppState;
use crate::auth::create_token;

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let admin = sqlx::query!(
        "SELECT id, email, password_hash, full_name FROM admins WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let admin = admin.ok_or((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;
    
    let valid = verify(&payload.password, &admin.password_hash)
        .map_err(|e| {
            eprintln!("Verify error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
    }
    
    let token = create_token(&admin.id, &state.jwt_secret)
        .map_err(|e| {
            eprintln!("JWT error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    Ok(Json(LoginResponse {
        token,
        admin_id: admin.id,
        full_name: admin.full_name,
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateAdminRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| {
            eprintln!("Hash error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let result = sqlx::query!(
        "INSERT INTO admins (id, email, password_hash, full_name) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        payload.email,
        password_hash,
        payload.full_name
    )
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Insert error: {}", e);
            if e.to_string().contains("unique constraint") {
                Err((StatusCode::CONFLICT, "Email already exists".to_string()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}