use axum::{extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;

use crate::models::user::UserResponse;
use crate::AppState;

/// Get user profile by ID.
pub async fn get_profile(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = state
        .photo_service
        .get_user(id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                (StatusCode::NOT_FOUND, "User not found".to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        })?;

    Ok(Json(user))
}
