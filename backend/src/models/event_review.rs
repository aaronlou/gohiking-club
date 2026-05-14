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

impl EventReview {
    pub fn validate_content(content: &str) -> Result<(), &'static str> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err("感想内容不能为空");
        }
        if trimmed.len() > 5000 {
            return Err("感想内容不能超过5000个字符");
        }
        Ok(())
    }

    pub fn validate_rating(rating: Option<i32>) -> Result<(), &'static str> {
        if let Some(r) = rating {
            if r < 1 || r > 5 {
                return Err("评分必须在1到5之间");
            }
        }
        Ok(())
    }
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
