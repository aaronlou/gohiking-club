use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::filters::PhotoFilter;
use crate::models::photo::PhotoResponse;
use crate::AppState;

/// Upload a photo — requires auth.
pub async fn upload(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<PhotoResponse>, (StatusCode, String)> {
    let mut file_data = None;
    let mut title = None;
    let mut description = None;
    let mut event_id = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("photo") => {
                file_data = Some(field.bytes().await.map_err(|e| {
                    (StatusCode::BAD_REQUEST, format!("Failed to read file: {e}"))
                })?);
            }
            Some("title") => {
                title = Some(field.text().await.unwrap_or_default());
            }
            Some("description") => {
                description = Some(field.text().await.unwrap_or_default());
            }
            Some("event_id") => {
                let val = field.text().await.unwrap_or_default();
                event_id = Uuid::parse_str(&val).ok();
            }
            _ => {}
        }
    }

    let data = file_data.ok_or((StatusCode::BAD_REQUEST, "Missing photo field".to_string()))?;

    let photo = state
        .photo_service
        .upload_photo(auth_user.id, data.to_vec(), title, description, event_id)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Upload failed: {e}"))
        })?;

    Ok(Json(photo))
}

/// List photos with optional filters.
pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<PhotoFilter>,
) -> Result<Json<Vec<PhotoResponse>>, (StatusCode, String)> {
    let photos = state
        .photo_service
        .list_photos(filter)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photos))
}

/// Get a single photo by ID.
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PhotoResponse>, (StatusCode, String)> {
    let photo = state
        .photo_service
        .get_photo(id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                (StatusCode::NOT_FOUND, "Photo not found".to_string())
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        })?;

    Ok(Json(photo))
}

/// Delete a photo — checks ownership.
pub async fn delete(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let photo = state
        .photo_service
        .get_photo(id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Photo not found".to_string()))?;

    if photo.user_id != auth_user.id {
        return Err((StatusCode::FORBIDDEN, "Not your photo".to_string()));
    }

    state
        .photo_service
        .delete_photo(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
