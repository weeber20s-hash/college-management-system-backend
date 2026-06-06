use axum::{Json, http::StatusCode, extract::State};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::config::AppState;
use crate::extractors::AdminId;
use crate::models::admin::{AdminProfile, UpdateProfileRequest, ChangePasswordRequest};

pub async fn get_profile(
    State(state): State<AppState>,
    admin_id: AdminId,
) -> Result<Json<AdminProfile>, (StatusCode, String)> {
    let admin_uuid = Uuid::parse_str(&admin_id.0)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid admin ID".to_string()))?;
    
    let admin = sqlx::query_as::<_, AdminProfile>(
        "SELECT id, email, full_name, created_at FROM admins WHERE id = $1"
    )
    .bind(admin_uuid)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get profile error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    match admin {
        Some(profile) => Ok(Json(profile)),
        None => Err((StatusCode::NOT_FOUND, "Admin not found".to_string())),
    }
}

pub async fn update_profile(
    State(state): State<AppState>,
    admin_id: AdminId,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<AdminProfile>, (StatusCode, String)> {
    let admin_uuid = Uuid::parse_str(&admin_id.0)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid admin ID".to_string()))?;
    
    let existing = sqlx::query!(
        "SELECT id, email, full_name FROM admins WHERE id = $1",
        admin_uuid
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Check admin error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let existing = match existing {
        Some(admin) => admin,
        None => return Err((StatusCode::NOT_FOUND, "Admin not found".to_string())),
    };
    
    let new_full_name = payload.full_name.unwrap_or(existing.full_name);
    let new_email = payload.email.unwrap_or(existing.email.clone());
    
    if new_email != existing.email {
        let email_taken = sqlx::query!(
            "SELECT id FROM admins WHERE email = $1 AND id != $2",
            new_email,
            admin_uuid
        )
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Check email error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        
        if email_taken.is_some() {
            return Err((StatusCode::CONFLICT, "Email already in use".to_string()));
        }
    }

    let updated = sqlx::query_as::<_, AdminProfile>(
        "UPDATE admins SET full_name = $1, email = $2 WHERE id = $3 
         RETURNING id, email, full_name, created_at"
    )
    .bind(new_full_name)
    .bind(new_email)
    .bind(admin_uuid)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Update profile error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(Json(updated))
}

pub async fn change_password(
    State(state): State<AppState>,
    admin_id: AdminId,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let admin_uuid = Uuid::parse_str(&admin_id.0)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid admin ID".to_string()))?;
    
    let admin = sqlx::query!(
        "SELECT password_hash FROM admins WHERE id = $1",
        admin_uuid
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Get admin error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let admin = match admin {
        Some(a) => a,
        None => return Err((StatusCode::NOT_FOUND, "Admin not found".to_string())),
    };

    let valid = verify(&payload.current_password, &admin.password_hash)
        .map_err(|e| {
            eprintln!("Password verify error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Current password is incorrect".to_string()));
    }
    
    let new_password_hash = hash(&payload.new_password, DEFAULT_COST)
        .map_err(|e| {
            eprintln!("Hash error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    sqlx::query!(
        "UPDATE admins SET password_hash = $1 WHERE id = $2",
        new_password_hash,
        admin_uuid
    )
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Update password error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    Ok(StatusCode::OK)
}