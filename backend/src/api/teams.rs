use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::models::event::EventResponse;
use crate::models::team::{
    CreateTeamRequest, Team, TeamMemberResponse, TeamResponse, UpdateTeamRequest,
};
use crate::repositories::team_repository::TeamRepository;
use crate::AppState;

#[derive(Deserialize)]
pub struct TeamFilter {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
struct TeamMemberRow {
    user_id: Uuid,
    username: String,
    avatar_url: Option<String>,
    role: String,
    joined_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    // 1. Domain validation (充血模型)
    Team::validate_name(&req.name)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.into()))?;
    Team::validate_slug(&req.slug)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.into()))?;

    let slug = Team::normalize_slug(&req.slug);

    // 2. Repository layer
    let repo = TeamRepository::new(&state.pool);

    if repo.exists_by_slug(&slug).await.map_err(internal_error)? {
        return Err((StatusCode::CONFLICT, "Team slug already taken".into()));
    }

    let team = repo
        .create_with_creator(&req.name, &slug, req.description.as_deref(), auth_user.id)
        .await
        .map_err(internal_error)?;

    Ok(Json(TeamResponse::from((team, 1, 0))))
}

pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<TeamFilter>,
) -> Result<Json<Vec<TeamResponse>>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0);

    let teams = repo
        .list(filter.status.as_deref(), limit, offset)
        .await
        .map_err(internal_error)?;

    let mut result = Vec::with_capacity(teams.len());
    for team in teams {
        let members = repo.get_member_count(team.id).await.unwrap_or(0);
        let events = repo.get_event_count(team.id).await.unwrap_or(0);
        result.push(TeamResponse::from((team, members, events)));
    }

    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let team = repo
        .find_by_id(id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Team not found".to_string()))?;

    let members = repo.get_member_count(team.id).await.unwrap_or(0);
    let events = repo.get_event_count(team.id).await.unwrap_or(0);
    Ok(Json(TeamResponse::from((team, members, events))))
}

pub async fn update(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can update".into()));
    }

    let team = repo
        .update(id, req.name.as_deref(), req.description.as_deref(), req.logo_url.as_deref(), req.cover_url.as_deref())
        .await
        .map_err(internal_error)?;

    let members = repo.get_member_count(team.id).await.unwrap_or(0);
    let events = repo.get_event_count(team.id).await.unwrap_or(0);
    Ok(Json(TeamResponse::from((team, members, events))))
}

pub async fn join(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let added = repo
        .add_member(id, auth_user.id, "member")
        .await
        .map_err(internal_error)?;

    if !added {
        return Err((StatusCode::CONFLICT, "Already a member".into()));
    }

    Ok(StatusCode::OK)
}

pub async fn leave(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);
    repo.remove_member(id, auth_user.id)
        .await
        .map_err(internal_error)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_members(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TeamMemberResponse>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, TeamMemberRow>(
        r#"
        SELECT u.id as user_id, u.username, u.avatar_url, tm.role, tm.joined_at
        FROM team_members tm
        JOIN users u ON tm.user_id = u.id
        WHERE tm.team_id = $1
        ORDER BY tm.joined_at ASC
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        rows
            .into_iter()
            .map(|r| TeamMemberResponse {
                user_id: r.user_id,
                username: r.username,
                avatar_url: r.avatar_url,
                role: r.role,
                joined_at: r.joined_at,
            })
            .collect(),
    ))
}

pub async fn get_events(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<EventResponse>>, (StatusCode, String)> {
    use crate::repositories::event_repository::EventRepository;

    let events = sqlx::query_as::<_, crate::models::event::Event>(
        r#"
        SELECT * FROM events
        WHERE team_id = $1 AND status = 'active'
        ORDER BY date DESC NULLS LAST, created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let event_repo = EventRepository::new(&state.pool);
    let mut result = Vec::with_capacity(events.len());
    for event in events {
        let members = event_repo.get_member_count(event.id).await.unwrap_or(0);
        let photos = event_repo.get_photo_count(event.id).await.unwrap_or(0);
        let reviews = event_repo.get_review_count(event.id).await.unwrap_or(0);
        result.push(EventResponse::from((event, members, photos, reviews)));
    }

    Ok(Json(result))
}

fn internal_error(e: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
