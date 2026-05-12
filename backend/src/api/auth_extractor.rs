use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::AppState;

/// Authenticated user extracted from the Authorization header.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header format".to_string()));
        }

        let token = &auth_header[7..];

        let user_id = state
            .auth_service
            .verify_token(token)
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        Ok(AuthenticatedUser { id: user_id })
    }
}
