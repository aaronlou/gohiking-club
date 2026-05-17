use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::models::event_review::{
    CreateEventReviewRequest, EventReviewResponse,
};
use crate::AppState;

#[derive(Debug, sqlx::FromRow)]
struct ReviewRow {
    id: Uuid,
    event_id: Uuid,
    user_id: Uuid,
    content: String,
    rating: Option<i32>,
    created_at: DateTime<Utc>,
    username: String,
    avatar_url: Option<String>,
}

pub async fn create(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Json(req): Json<CreateEventReviewRequest>,
) -> Result<Json<EventReviewResponse>, (StatusCode, String)> {
    if req.content.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Content is required".into()));
    }

    if let Some(rating) = req.rating {
        if rating < 1 || rating > 5 {
            return Err((StatusCode::BAD_REQUEST, "Rating must be between 1 and 5".into()));
        }
    }

    // Check if user joined the event
    let joined: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM event_members WHERE event_id = $1 AND user_id = $2",
    )
    .bind(event_id)
    .bind(auth_user.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if joined.0 == 0 {
        return Err((
            StatusCode::FORBIDDEN,
            "You must join the event before writing a review".into(),
        ));
    }

    let row = sqlx::query_as::<_, ReviewRow>(
        r#"
        WITH inserted AS (
            INSERT INTO event_reviews (event_id, user_id, content, rating)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        )
        SELECT i.*, u.username, u.avatar_url
        FROM inserted i
        JOIN users u ON i.user_id = u.id
        "#,
    )
    .bind(event_id)
    .bind(auth_user.id)
    .bind(&req.content)
    .bind(req.rating)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(row)))
}

pub async fn list(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<EventReviewResponse>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, ReviewRow>(
        r#"
        SELECT er.*, u.username, u.avatar_url
        FROM event_reviews er
        JOIN users u ON er.user_id = u.id
        WHERE er.event_id = $1
        ORDER BY er.created_at DESC
        "#,
    )
    .bind(event_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows.into_iter().map(to_response).collect()))
}

pub async fn delete(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((_event_id, review_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        "DELETE FROM event_reviews WHERE id = $1 AND user_id = $2",
    )
    .bind(review_id)
    .bind(auth_user.id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Review not found or not yours".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn to_response(row: ReviewRow) -> EventReviewResponse {
    EventReviewResponse {
        id: row.id,
        event_id: row.event_id,
        user_id: row.user_id,
        username: row.username,
        avatar_url: row.avatar_url,
        content: row.content,
        rating: row.rating,
        created_at: row.created_at,
    }
}
