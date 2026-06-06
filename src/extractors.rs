use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AdminId(pub String);

impl<S> FromRequestParts<S> for AdminId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<String>()
            .cloned()
            .map(AdminId)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}