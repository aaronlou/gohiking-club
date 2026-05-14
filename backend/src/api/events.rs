use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::models::event::{CreateEventRequest, Event, EventResponse};
use crate::models::photo::{Photo, PhotoResponse};
use crate::repositories::event_repository::EventRepository;
use crate::AppState;

#[derive(Deserialize)]
pub struct EventFilter {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, (StatusCode, String)> {
    // Domain validation
    Event::validate_title(&req.title)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.into()))?;

    let repo = EventRepository::new(&state.pool);
    let event = repo
        .create_with_creator(
            &req.title,
            req.description.as_deref(),
            req.location.as_deref(),
            req.date.as_ref(),
            auth_user.id,
            req.team_id,
        )
        .await
        .map_err(internal_error)?;

    Ok(Json(EventResponse::from((event, 1, 0, 0))))
}

pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<EventFilter>,
) -> Result<Json<Vec<EventResponse>>, (StatusCode, String)> {
    let repo = EventRepository::new(&state.pool);
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0);

    let events = repo
        .list(filter.status.as_deref(), limit, offset)
        .await
        .map_err(internal_error)?;

    let mut result = Vec::with_capacity(events.len());
    for event in events {
        let members = repo.get_member_count(event.id).await.unwrap_or(0);
        let photos = repo.get_photo_count(event.id).await.unwrap_or(0);
        let reviews = repo.get_review_count(event.id).await.unwrap_or(0);
        result.push(EventResponse::from((event, members, photos, reviews)));
    }

    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, (StatusCode, String)> {
    let repo = EventRepository::new(&state.pool);

    let event = repo
        .find_by_id(id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Event not found".to_string()))?;

    let members = repo.get_member_count(event.id).await.unwrap_or(0);
    let photos = repo.get_photo_count(event.id).await.unwrap_or(0);
    let reviews = repo.get_review_count(event.id).await.unwrap_or(0);
    Ok(Json(EventResponse::from((event, members, photos, reviews))))
}

pub async fn join(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = EventRepository::new(&state.pool);
    let added = repo
        .add_member(id, auth_user.id, "member")
        .await
        .map_err(internal_error)?;

    if !added {
        return Err((StatusCode::CONFLICT, "Already joined".into()));
    }

    Ok(StatusCode::OK)
}

pub async fn get_photos(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PhotoResponse>>, (StatusCode, String)> {
    let photos = sqlx::query_as::<_, Photo>(
        r#"
        SELECT * FROM photos
        WHERE event_id = $1 AND status = 'approved'
        ORDER BY ai_score DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photos.into_iter().map(Into::into).collect()))
}

fn internal_error(e: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
