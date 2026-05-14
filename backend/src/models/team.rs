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

#[derive(Debug, Serialize)]
pub struct TeamMemberResponse {
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}
