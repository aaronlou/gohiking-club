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
    pub distance_km: Option<f64>,
    pub elevation_gain_m: Option<i32>,
    pub disclaimer: Option<String>,
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

impl Event {
    pub fn validate_title(title: &str) -> Result<(), &'static str> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err("活动标题不能为空");
        }
        if trimmed.len() > 200 {
            return Err("活动标题不能超过200个字符");
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date: Option<NaiveDate>,
    pub team_id: Option<Uuid>,
    pub distance_km: Option<f64>,
    pub elevation_gain_m: Option<i32>,
    pub disclaimer: Option<String>,
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
    pub distance_km: Option<f64>,
    pub elevation_gain_m: Option<i32>,
    pub disclaimer: Option<String>,
    pub member_count: i64,
    pub photo_count: i64,
    pub review_count: i64,
    pub is_team_member: bool,
    pub created_at: DateTime<Utc>,
}

impl EventResponse {
    pub fn from_event(
        e: Event,
        members: i64,
        photos: i64,
        reviews: i64,
        is_team_member: bool,
    ) -> Self {
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
            distance_km: e.distance_km,
            elevation_gain_m: e.elevation_gain_m,
            disclaimer: e.disclaimer,
            member_count: members,
            photo_count: photos,
            review_count: reviews,
            is_team_member,
            created_at: e.created_at,
        }
    }
}
