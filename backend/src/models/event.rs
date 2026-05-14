use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date: Option<NaiveDate>,
    pub cover_url: Option<String>,
    pub created_by: Uuid,
    pub team_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventMember {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date: Option<NaiveDate>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date: Option<NaiveDate>,
    pub cover_url: Option<String>,
    pub created_by: Uuid,
    pub team_id: Option<Uuid>,
    pub status: String,
    pub member_count: i64,
    pub photo_count: i64,
    pub review_count: i64,
    pub created_at: DateTime<Utc>,
}

impl From<(Event, i64, i64, i64)> for EventResponse {
    fn from((e, members, photos, reviews): (Event, i64, i64, i64)) -> Self {
        Self {
            id: e.id,
            title: e.title,
            description: e.description,
            location: e.location,
            date: e.date,
            cover_url: e.cover_url,
            created_by: e.created_by,
            team_id: e.team_id,
            status: e.status,
            member_count: members,
            photo_count: photos,
            review_count: reviews,
            created_at: e.created_at,
        }
    }
}
