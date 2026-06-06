use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use crate::config::AppState;
use crate::auth::verify_token;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    let auth_str = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = &auth_str[7..];
    
    let claims = match verify_token(token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    req.extensions_mut().insert(claims.sub);
    
    Ok(next.run(req).await)
}