use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::photo::PhotoResponse;
use crate::AppState;

#[derive(Deserialize)]
pub struct PhotoFilter {
    pub status: Option<String>,
    pub min_score: Option<f64>,
    pub user_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Upload a photo — optionally tied to an event.
pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<PhotoResponse>, (StatusCode, String)> {
    let user_id = Uuid::new_v4(); // TODO: extract from auth context

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
        .upload_photo(user_id, data.to_vec(), title, description, event_id)
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

/// Delete a photo.
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .photo_service
        .delete_photo(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
