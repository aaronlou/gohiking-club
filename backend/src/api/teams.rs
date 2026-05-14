use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::models::event::EventResponse;
use crate::models::team::{
    CreateTeamRequest, Team, TeamMemberResponse, TeamResponse, UpdateTeamRequest,
};
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
    joined_at: DateTime<Utc>,
}

async fn get_team_counts(pool: &sqlx::PgPool, team_id: Uuid) -> (i64, i64) {
    let members: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM team_members WHERE team_id = $1")
        .bind(team_id)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    let events: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE team_id = $1")
        .bind(team_id)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    (members.0, events.0)
}

pub async fn create(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    if req.name.trim().is_empty() || req.slug.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name and slug are required".into()));
    }

    let slug = req.slug.trim().to_lowercase().replace(" ", "-");

    // Check slug uniqueness
    let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM teams WHERE slug = $1")
        .bind(&slug)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing.0 > 0 {
        return Err((StatusCode::CONFLICT, "Team slug already taken".into()));
    }

    let team = sqlx::query_as::<_, Team>(
        r#"
        INSERT INTO teams (name, slug, description, created_by)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&slug)
    .bind(&req.description)
    .bind(auth_user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Creator auto-joins as admin
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role) VALUES ($1, $2, 'admin')",
    )
    .bind(team.id)
    .bind(auth_user.id)
    .execute(&state.pool)
    .await
    .ok();

    Ok(Json(TeamResponse::from((team, 1, 0))))
}

pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<TeamFilter>,
) -> Result<Json<Vec<TeamResponse>>, (StatusCode, String)> {
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0);

    let teams = sqlx::query_as::<_, Team>(
        r#"
        SELECT * FROM teams
        WHERE ($1::text IS NULL OR status = $1)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(filter.status.as_deref())
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut result = Vec::with_capacity(teams.len());
    for team in teams {
        let (members, events) = get_team_counts(&state.pool, team.id).await;
        result.push(TeamResponse::from((team, members, events)));
    }

    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    let team = sqlx::query_as::<_, Team>("SELECT * FROM teams WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Team not found".to_string()))?;

    let (members, events) = get_team_counts(&state.pool, team.id).await;
    Ok(Json(TeamResponse::from((team, members, events))))
}

pub async fn update(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, (StatusCode, String)> {
    // Check if user is admin
    let is_admin: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM team_members WHERE team_id = $1 AND user_id = $2 AND role = 'admin'",
    )
    .bind(id)
    .bind(auth_user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if is_admin.0 == 0 {
        return Err((StatusCode::FORBIDDEN, "Only team admin can update".into()));
    }

    let team = sqlx::query_as::<_, Team>(
        r#"
        UPDATE teams
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            logo_url = COALESCE($4, logo_url),
            cover_url = COALESCE($5, cover_url),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.logo_url)
    .bind(&req.cover_url)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (members, events) = get_team_counts(&state.pool, team.id).await;
    Ok(Json(TeamResponse::from((team, members, events))))
}

pub async fn join(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        "INSERT INTO team_members (team_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(id)
    .bind(auth_user.id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::CONFLICT, "Already a member".into()));
    }

    Ok(StatusCode::OK)
}

pub async fn leave(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
        .bind(id)
        .bind(auth_user.id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut result = Vec::with_capacity(events.len());
    for event in events {
        let members: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_members WHERE event_id = $1")
            .bind(event.id)
            .fetch_one(&state.pool)
            .await
            .unwrap_or((0,));
        let photos: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM photos WHERE event_id = $1")
            .bind(event.id)
            .fetch_one(&state.pool)
            .await
            .unwrap_or((0,));
        let reviews: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_reviews WHERE event_id = $1")
            .bind(event.id)
            .fetch_one(&state.pool)
            .await
            .unwrap_or((0,));
        result.push(EventResponse::from((event, members.0, photos.0, reviews.0)));
    }

    Ok(Json(result))
}
