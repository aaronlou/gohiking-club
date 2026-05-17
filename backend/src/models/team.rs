use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub cover_url: Option<String>,
    pub created_by: Uuid,
    pub status: String,
    pub default_disclaimer: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMember {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub cover_url: Option<String>,
    pub default_disclaimer: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub cover_url: Option<String>,
    pub created_by: Uuid,
    pub status: String,
    pub member_count: i64,
    pub event_count: i64,
    pub created_at: DateTime<Utc>,
}

impl Team {
    pub fn normalize_slug(slug: &str) -> String {
        slug.trim().to_lowercase().replace(" ", "-")
    }

    pub fn validate_name(name: &str) -> Result<(), &'static str> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("团队名称不能为空");
        }
        if trimmed.len() > 100 {
            return Err("团队名称不能超过100个字符");
        }
        Ok(())
    }

    pub fn validate_slug(slug: &str) -> Result<(), &'static str> {
        let trimmed = slug.trim();
        if trimmed.is_empty() {
            return Err("团队标识不能为空");
        }
        if trimmed.len() > 100 {
            return Err("团队标识不能超过100个字符");
        }
        if trimmed.contains(' ') {
            return Err("团队标识不能包含空格");
        }
        Ok(())
    }
}

impl From<(Team, i64, i64)> for TeamResponse {
    fn from((t, members, events): (Team, i64, i64)) -> Self {
        Self {
            id: t.id,
            name: t.name,
            slug: t.slug,
            description: t.description,
            logo_url: t.logo_url,
            cover_url: t.cover_url,
            created_by: t.created_by,
            status: t.status,
            member_count: members,
            event_count: events,
            created_at: t.created_at,
        }
    }
}

// ── Team Invitation ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamInvitation {
    pub id: Uuid,
    pub team_id: Uuid,
    pub code: String,
    pub created_by: Uuid,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TeamInvitationResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub code: String,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<TeamInvitation> for TeamInvitationResponse {
    fn from(inv: TeamInvitation) -> Self {
        Self {
            id: inv.id,
            team_id: inv.team_id,
            code: inv.code,
            max_uses: inv.max_uses,
            used_count: inv.used_count,
            expires_at: inv.expires_at,
            status: inv.status,
            created_at: inv.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateInvitationRequest {
    pub max_uses: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
}

// ── Team Join Request ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamJoinRequest {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub invitation_code: Option<String>,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct JoinRequestRow {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub invitation_code: Option<String>,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamJoinRequestResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub invitation_code: Option<String>,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ApplyJoinRequest {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveJoinRequest {
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct TeamMemberResponse {
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}
