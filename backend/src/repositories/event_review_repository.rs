use sqlx::PgPool;
use uuid::Uuid;

use crate::models::event_review::EventReview;

pub struct EventReviewRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> EventReviewRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_or_update(
        &self,
        event_id: Uuid,
        user_id: Uuid,
        content: &str,
        rating: Option<i32>,
    ) -> anyhow::Result<EventReview> {
        let review = sqlx::query_as::<_, EventReview>(
            r#"
            INSERT INTO event_reviews (event_id, user_id, content, rating)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (event_id, user_id) DO UPDATE SET
                content = EXCLUDED.content,
                rating = EXCLUDED.rating,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(event_id)
        .bind(user_id)
        .bind(content)
        .bind(rating)
        .fetch_one(self.pool)
        .await?;
        Ok(review)
    }

    pub async fn list_by_event(&self, event_id: Uuid) -> anyhow::Result<Vec<EventReview>> {
        let reviews = sqlx::query_as::<_, EventReview>(
            "SELECT * FROM event_reviews WHERE event_id = $1 ORDER BY created_at DESC",
        )
        .bind(event_id)
        .fetch_all(self.pool)
        .await?;
        Ok(reviews)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query(
            "DELETE FROM event_reviews WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn count_by_event(&self, event_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_reviews WHERE event_id = $1")
            .bind(event_id)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0)
    }
}
