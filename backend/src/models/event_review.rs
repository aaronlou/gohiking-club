use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventReview {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub rating: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventReviewRequest {
    pub content: String,
    pub rating: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventReviewRequest {
    pub content: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct EventReviewResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub content: String,
    pub rating: Option<i32>,
    pub created_at: DateTime<Utc>,
}
