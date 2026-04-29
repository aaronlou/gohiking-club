use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::models::user::UserResponse;
use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
}

/// Register a new user.
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    if req.username.is_empty() || req.email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username and email required".into()));
    }

    let user = state
        .photo_service
        .create_user(&req.username, &req.email)
        .await
        .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;

    Ok(Json(user))
}
