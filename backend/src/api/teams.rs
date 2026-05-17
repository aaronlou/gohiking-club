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
    ApplyJoinRequest, ApproveJoinRequest, CreateInvitationRequest, CreateTeamRequest,
    Team, TeamInvitationResponse, TeamJoinRequestResponse, TeamMemberResponse,
    TeamResponse, UpdateTeamRequest,
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
    Team::validate_name(&req.name)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.into()))?;
    Team::validate_slug(&req.slug)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.into()))?;

    let slug = Team::normalize_slug(&req.slug);
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

    // Update default_disclaimer separately if provided
    if req.default_disclaimer.is_some() {
        sqlx::query("UPDATE teams SET default_disclaimer = $1, updated_at = NOW() WHERE id = $2")
            .bind(&req.default_disclaimer)
            .bind(id)
            .execute(&state.pool)
            .await
            .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

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
        result.push(EventResponse::from_event(event, members, photos, reviews, false));
    }

    Ok(Json(result))
}

// ── Invitations ──

pub async fn create_invitation(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateInvitationRequest>,
) -> Result<Json<TeamInvitationResponse>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can create invitations".into()));
    }

    let code = format!("{}{}", uuid::Uuid::new_v4().simple(), uuid::Uuid::new_v4().simple());

    let inv = repo
        .create_invitation(id, &code, auth_user.id, req.max_uses, req.expires_at)
        .await
        .map_err(internal_error)?;

    Ok(Json(TeamInvitationResponse::from(inv)))
}

pub async fn list_invitations(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TeamInvitationResponse>>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can view invitations".into()));
    }

    let invs = repo
        .list_invitations(id)
        .await
        .map_err(internal_error)?;

    Ok(Json(invs.into_iter().map(Into::into).collect()))
}

pub async fn get_invitation_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let inv = repo
        .find_invitation_by_code(&code)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Invitation not found".to_string()))?;

    if inv.status != "active" {
        return Err((StatusCode::GONE, "Invitation is no longer active".into()));
    }

    if let Some(expires_at) = inv.expires_at {
        if chrono::Utc::now() > expires_at {
            return Err((StatusCode::GONE, "Invitation has expired".into()));
        }
    }

    if let Some(max_uses) = inv.max_uses {
        if inv.used_count >= max_uses {
            return Err((StatusCode::GONE, "Invitation has reached max uses".into()));
        }
    }

    let team = repo
        .find_by_id(inv.team_id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Team not found".to_string()))?;

    let members = repo.get_member_count(team.id).await.unwrap_or(0);
    let events = repo.get_event_count(team.id).await.unwrap_or(0);

    Ok(Json(serde_json::json!({
        "invitation": TeamInvitationResponse::from(inv),
        "team": TeamResponse::from((team, members, events)),
    })))
}

// ── Join Requests ──

pub async fn apply_join(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(code): Path<String>,
    Json(req): Json<ApplyJoinRequest>,
) -> Result<Json<TeamJoinRequestResponse>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let inv = repo
        .find_invitation_by_code(&code)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Invitation not found".to_string()))?;

    if inv.status != "active" {
        return Err((StatusCode::GONE, "Invitation is no longer active".into()));
    }

    if let Some(expires_at) = inv.expires_at {
        if chrono::Utc::now() > expires_at {
            return Err((StatusCode::GONE, "Invitation has expired".into()));
        }
    }

    if let Some(max_uses) = inv.max_uses {
        if inv.used_count >= max_uses {
            return Err((StatusCode::GONE, "Invitation has reached max uses".into()));
        }
    }

    // Check if already a member
    let is_member = repo
        .is_member(inv.team_id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if is_member {
        return Err((StatusCode::CONFLICT, "Already a member".into()));
    }

    let join_req = repo
        .create_join_request(inv.team_id, auth_user.id, Some(&code), req.message.as_deref())
        .await
        .map_err(internal_error)?;

    // Get user info for response
    let user_row: (String, Option<String>) = sqlx::query_as(
        "SELECT username, avatar_url FROM users WHERE id = $1"
    )
    .bind(auth_user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TeamJoinRequestResponse {
        id: join_req.id,
        team_id: join_req.team_id,
        user_id: join_req.user_id,
        username: user_row.0,
        avatar_url: user_row.1,
        invitation_code: join_req.invitation_code,
        message: join_req.message,
        status: join_req.status,
        created_at: join_req.created_at,
    }))
}

pub async fn list_join_requests(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TeamJoinRequestResponse>>, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can view join requests".into()));
    }

    let rows = repo
        .list_pending_join_requests(id)
        .await
        .map_err(internal_error)?;

    Ok(Json(
        rows
            .into_iter()
            .map(|r| TeamJoinRequestResponse {
                id: r.id,
                team_id: r.team_id,
                user_id: r.user_id,
                username: r.username,
                avatar_url: r.avatar_url,
                invitation_code: r.invitation_code,
                message: r.message,
                status: r.status,
                created_at: r.created_at,
            })
            .collect(),
    ))
}

pub async fn approve_join_request(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApproveJoinRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can approve requests".into()));
    }

    let join_req = repo
        .find_join_request_by_id(req.request_id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Join request not found".to_string()))?;

    if join_req.team_id != id {
        return Err((StatusCode::FORBIDDEN, "Request does not belong to this team".into()));
    }

    if join_req.status != "pending" {
        return Err((StatusCode::CONFLICT, "Request is not pending".into()));
    }

    // Add member
    repo
        .add_member(id, join_req.user_id, "member")
        .await
        .map_err(internal_error)?;

    // Update request status
    repo
        .update_join_request_status(req.request_id, "approved")
        .await
        .map_err(internal_error)?;

    // Increment invitation used count
    if let Some(code) = &join_req.invitation_code {
        repo.increment_invitation_used(code).await.ok();
    }

    Ok(StatusCode::OK)
}

pub async fn reject_join_request(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApproveJoinRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let repo = TeamRepository::new(&state.pool);

    let is_admin = repo
        .is_admin(id, auth_user.id)
        .await
        .map_err(internal_error)?;

    if !is_admin {
        return Err((StatusCode::FORBIDDEN, "Only team admin can reject requests".into()));
    }

    let join_req = repo
        .find_join_request_by_id(req.request_id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Join request not found".to_string()))?;

    if join_req.team_id != id {
        return Err((StatusCode::FORBIDDEN, "Request does not belong to this team".into()));
    }

    if join_req.status != "pending" {
        return Err((StatusCode::CONFLICT, "Request is not pending".into()));
    }

    repo
        .update_join_request_status(req.request_id, "rejected")
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::OK)
}

fn internal_error(e: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
