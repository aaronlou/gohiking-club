use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "photo_status", rename_all = "lowercase")]
pub enum PhotoStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Photo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub storage_key: Option<String>,
    pub thumbnail_storage_key: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub ai_score: f64,
    pub ai_feedback: Option<serde_json::Value>,
    pub status: PhotoStatus,
    pub event_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePhotoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PhotoResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub ai_score: f64,
    pub ai_feedback: Option<serde_json::Value>,
    pub status: PhotoStatus,
    pub event_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<Photo> for PhotoResponse {
    fn from(p: Photo) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            url: p.url,
            thumbnail_url: p.thumbnail_url,
            title: p.title,
            description: p.description,
            ai_score: p.ai_score,
            ai_feedback: p.ai_feedback,
            status: p.status,
            event_id: p.event_id,
            created_at: p.created_at,
        }
    }
}


