use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::models::event::{CreateEventRequest, Event, EventResponse};
use crate::repositories::team_repository::TeamRepository;
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

    // If team_id is provided, verify the user is an admin of that team
    if let Some(team_id) = req.team_id {
        let team_repo = TeamRepository::new(&state.pool);
        let is_admin = team_repo
            .is_admin(team_id, auth_user.id)
            .await
            .map_err(internal_error)?;
        if !is_admin {
            return Err((StatusCode::FORBIDDEN, "Only team admin can create team events".into()));
        }
    }

    let repo = EventRepository::new(&state.pool);
    let event = repo
        .create_with_creator(
            &req.title,
            req.description.as_deref(),
            req.location.as_deref(),
            req.date.as_ref(),
            auth_user.id,
            req.team_id,
            req.distance_km,
            req.elevation_gain_m,
            req.disclaimer.as_deref(),
        )
        .await
        .map_err(internal_error)?;

    let members = repo.get_member_count(event.id).await.unwrap_or(0);
    let photos = repo.get_photo_count(event.id).await.unwrap_or(0);
    let reviews = repo.get_review_count(event.id).await.unwrap_or(0);
    let is_team_member = if let Some(team_id) = event.team_id {
        TeamRepository::new(&state.pool).is_member(team_id, auth_user.id).await.unwrap_or(false)
    } else {
        false
    };

    Ok(Json(EventResponse::from_event(event, members, photos, reviews, is_team_member)))
}

pub async fn list(
    auth_user: Option<AuthenticatedUser>,
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
        let is_team_member = if let (Some(team_id), Some(ref user)) = (event.team_id, &auth_user) {
            TeamRepository::new(&state.pool).is_member(team_id, user.id).await.unwrap_or(false)
        } else {
            false
        };
        result.push(EventResponse::from_event(event, members, photos, reviews, is_team_member));
    }

    Ok(Json(result))
}

pub async fn get(
    auth_user: Option<AuthenticatedUser>,
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

    let is_team_member = if let Some(team_id) = event.team_id {
        if let Some(ref user) = auth_user {
            TeamRepository::new(&state.pool).is_member(team_id, user.id).await.unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    };

    Ok(Json(EventResponse::from_event(event, members, photos, reviews, is_team_member)))
}

pub async fn join(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = EventRepository::new(&state.pool);

    let event = repo
        .find_by_id(id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Event not found".to_string()))?;

    // If event belongs to a team, user must be a team member
    if let Some(team_id) = event.team_id {
        let team_repo = TeamRepository::new(&state.pool);
        let is_team_member = team_repo
            .is_member(team_id, auth_user.id)
            .await
            .map_err(internal_error)?;

        if !is_team_member {
            return Err((StatusCode::FORBIDDEN, "Only team members can join this event".into()));
        }
    }

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
