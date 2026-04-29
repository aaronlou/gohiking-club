use sqlx::PgPool;
use uuid::Uuid;

use crate::api::photos::PhotoFilter;
use crate::infra::storage::Storage;
use crate::models::photo::{Photo, PhotoResponse, PhotoStatus};
use crate::models::user::{User, UserResponse};
use crate::services::scoring_service::ScoringService;

pub struct PhotoService {
    pool: PgPool,
    storage: Storage,
    scorer: ScoringService,
}

impl PhotoService {
    pub fn new(pool: PgPool, storage: Storage, scorer: ScoringService) -> Self {
        Self { pool, storage, scorer }
    }

    pub async fn upload_photo(
        &self,
        user_id: Uuid,
        data: Vec<u8>,
        title: Option<String>,
        description: Option<String>,
        event_id: Option<Uuid>,
    ) -> anyhow::Result<PhotoResponse> {
        // 1. Upload original to S3
        let key = self.storage.upload(data.clone(), "image/jpeg", &format!("photos/{}", user_id)).await?;
        let url = self.storage.public_url(&key);

        // 2. Generate thumbnail (simple resize via image crate)
        let thumbnail_key = format!("thumbnails/{}_{}", user_id, Uuid::new_v4());
        let thumbnail_url = self.storage.public_url(&thumbnail_key);
        // TODO: actual resize with image crate
        self.storage.upload(data.clone(), "image/jpeg", &thumbnail_key).await?;

        // 3. AI scoring
        let score_result = self.scorer.score(&data).await;

        let (ai_score, ai_feedback, status) = match score_result {
            Ok(result) => {
                let status = if result.overall >= self.scorer.threshold() {
                    PhotoStatus::Approved
                } else {
                    PhotoStatus::Rejected
                };
                (result.overall, Some(serde_json::Value::from(result)), status)
            }
            Err(e) => {
                tracing::warn!("AI scoring failed: {e}, defaulting to pending");
                (0.0, None, PhotoStatus::Pending)
            }
        };

        // 4. Save to database
        let photo = sqlx::query_as::<_, Photo>(
            r#"
            INSERT INTO photos (user_id, url, thumbnail_url, title, description, ai_score, ai_feedback, status, event_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&url)
        .bind(&thumbnail_url)
        .bind(&title)
        .bind(&description)
        .bind(ai_score)
        .bind(&ai_feedback)
        .bind(&status)
        .bind(event_id)
        .fetch_one(&self.pool)
        .await?;

        if status == PhotoStatus::Rejected {
            tracing::info!("Photo {} scored {:.1} — rejected (threshold: {:.1})", photo.id, ai_score, self.scorer.threshold());
        }

        Ok(PhotoResponse::from(photo))
    }

    pub async fn list_photos(&self, filter: PhotoFilter) -> anyhow::Result<Vec<PhotoResponse>> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(photos.into_iter().map(PhotoResponse::from).collect())
    }

    pub async fn get_photo(&self, id: Uuid) -> anyhow::Result<PhotoResponse> {
        let photo = sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("photo not found"))?;

        Ok(PhotoResponse::from(photo))
    }

    pub async fn delete_photo(&self, id: Uuid) -> anyhow::Result<()> {
        let photo = sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("photo not found"))?;

        // Delete from S3
        if let Some(key) = photo.url.split('/').last() {
            let _ = self.storage.delete(key).await;
        }

        sqlx::query("DELETE FROM photos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_user(&self, username: &str, email: &str) -> anyhow::Result<UserResponse> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(username)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            photo_count: 0,
            created_at: user.created_at,
        })
    }

    pub async fn get_user(&self, id: Uuid) -> anyhow::Result<UserResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let photo_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM photos WHERE user_id = $1 AND status = 'approved'",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            photo_count: photo_count.0,
            created_at: user.created_at,
        })
    }
}
