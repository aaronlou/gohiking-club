use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::event::{CreateEventRequest, Event, EventResponse};
use crate::models::photo::{Photo, PhotoResponse};
use crate::AppState;

#[derive(Deserialize)]
pub struct EventFilter {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

async fn get_counts(pool: &sqlx::PgPool, event_id: Uuid) -> (i64, i64) {
    let members: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM event_members WHERE event_id = $1",
    )
    .bind(event_id)
    .fetch_one(pool)
    .await
    .unwrap_or((0,));

    let photos: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM photos WHERE event_id = $1",
    )
    .bind(event_id)
    .fetch_one(pool)
    .await
    .unwrap_or((0,));

    (members.0, photos.0)
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, (StatusCode, String)> {
    let user_id = Uuid::new_v4(); // TODO: extract from auth

    if req.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title is required".into()));
    }

    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (title, description, location, date, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.location)
    .bind(&req.date)
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Creator auto-joins as admin
    sqlx::query(
        "INSERT INTO event_members (event_id, user_id, role) VALUES ($1, $2, 'admin')",
    )
    .bind(event.id)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .ok();

    Ok(Json(EventResponse::from((event, 1, 0))))
}

pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<EventFilter>,
) -> Result<Json<Vec<EventResponse>>, (StatusCode, String)> {
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0);

    let events = sqlx::query_as::<_, Event>(
        r#"
        SELECT * FROM events
        WHERE ($1::text IS NULL OR status = $1)
        ORDER BY date DESC NULLS LAST, created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(filter.status.as_deref())
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut result = Vec::with_capacity(events.len());
    for event in events {
        let (members, photos) = get_counts(&state.pool, event.id).await;
        result.push(EventResponse::from((event, members, photos)));
    }

    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, (StatusCode, String)> {
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Event not found".to_string()))?;

    let (members, photos) = get_counts(&state.pool, event.id).await;
    Ok(Json(EventResponse::from((event, members, photos))))
}

pub async fn join(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = Uuid::new_v4(); // TODO: extract from auth

    let result = sqlx::query(
        "INSERT INTO event_members (event_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(id)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::CONFLICT, "Already joined".into()));
    }

    Ok(StatusCode::OK)
}

pub async fn get_photos(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PhotoResponse>>, (StatusCode, String)> {
    let photos = sqlx::query_as::<_, Photo>(
        r#"
        SELECT * FROM photos
        WHERE event_id = $1 AND status = 'approved'
        ORDER BY ai_score DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(photos.into_iter().map(Into::into).collect()))
}
