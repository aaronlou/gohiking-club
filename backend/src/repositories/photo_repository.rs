use sqlx::PgPool;
use uuid::Uuid;

use crate::filters::PhotoFilter;
use crate::models::photo::{Photo, PhotoStatus};

pub struct PhotoRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PhotoRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        url: &str,
        thumbnail_url: Option<&str>,
        storage_key: Option<&str>,
        thumbnail_storage_key: Option<&str>,
        title: Option<&str>,
        description: Option<&str>,
        ai_score: f64,
        ai_feedback: Option<&serde_json::Value>,
        status: &PhotoStatus,
        event_id: Option<Uuid>,
    ) -> anyhow::Result<Photo> {
        let photo = sqlx::query_as::<_, Photo>(
            r#"
            INSERT INTO photos (user_id, url, thumbnail_url, storage_key, thumbnail_storage_key, title, description, ai_score, ai_feedback, status, event_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(url)
        .bind(thumbnail_url)
        .bind(storage_key)
        .bind(thumbnail_storage_key)
        .bind(title)
        .bind(description)
        .bind(ai_score)
        .bind(ai_feedback)
        .bind(status)
        .bind(event_id)
        .fetch_one(self.pool)
        .await?;
        Ok(photo)
    }

    pub async fn list(&self, filter: &PhotoFilter) -> anyhow::Result<Vec<Photo>> {
        let limit = filter.limit.unwrap_or(20).clamp(1, 100);
        let offset = filter.offset.unwrap_or(0);

        let photos = sqlx::query_as::<_, Photo>(
            r#"
            SELECT * FROM photos
            WHERE ($1::text IS NULL OR status::text = $1)
              AND ($2::float8 IS NULL OR ai_score >= $2)
              AND ($3::uuid IS NULL OR user_id = $3)
              AND ($4::uuid IS NULL OR event_id = $4)
            ORDER BY ai_score DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(filter.status.as_deref())
        .bind(filter.min_score)
        .bind(filter.user_id)
        .bind(filter.event_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;
        Ok(photos)
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Photo>> {
        let photo = sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;
        Ok(photo)
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM photos WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
