use sqlx::PgPool;
use uuid::Uuid;

use crate::models::event::Event;

pub struct EventRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> EventRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        title: &str,
        description: Option<&str>,
        location: Option<&str>,
        date: Option<&chrono::NaiveDate>,
        created_by: Uuid,
        team_id: Option<Uuid>,
        distance_km: Option<f64>,
        elevation_gain_m: Option<i32>,
        disclaimer: Option<&str>,
    ) -> anyhow::Result<Event> {
        let event = sqlx::query_as::<_, Event>(
            r#"
            INSERT INTO events (title, description, location, date, created_by, team_id, distance_km, elevation_gain_m, disclaimer)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(location)
        .bind(date)
        .bind(created_by)
        .bind(team_id)
        .bind(distance_km)
        .bind(elevation_gain_m)
        .bind(disclaimer)
        .fetch_one(self.pool)
        .await?;
        Ok(event)
    }

    /// Create event and add creator as admin atomically.
    pub async fn create_with_creator(
        &self,
        title: &str,
        description: Option<&str>,
        location: Option<&str>,
        date: Option<&chrono::NaiveDate>,
        created_by: Uuid,
        team_id: Option<Uuid>,
        distance_km: Option<f64>,
        elevation_gain_m: Option<i32>,
        disclaimer: Option<&str>,
    ) -> anyhow::Result<Event> {
        let mut tx = self.pool.begin().await?;

        let event = sqlx::query_as::<_, Event>(
            r#"
            INSERT INTO events (title, description, location, date, created_by, team_id, distance_km, elevation_gain_m, disclaimer)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(location)
        .bind(date)
        .bind(created_by)
        .bind(team_id)
        .bind(distance_km)
        .bind(elevation_gain_m)
        .bind(disclaimer)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO event_members (event_id, user_id, role) VALUES ($1, $2, 'admin')",
        )
        .bind(event.id)
        .bind(created_by)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(event)
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Event>> {
        let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;
        Ok(event)
    }

    pub async fn list(
        &self,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            r#"
            SELECT * FROM events
            WHERE ($1::text IS NULL OR status = $1)
            ORDER BY date DESC NULLS LAST, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;
        Ok(events)
    }

    pub async fn add_member(&self, event_id: Uuid, user_id: Uuid, role: &str) -> anyhow::Result<bool> {
        let result = sqlx::query(
            "INSERT INTO event_members (event_id, user_id, role) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        )
        .bind(event_id)
        .bind(user_id)
        .bind(role)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_member_count(&self, event_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM event_members WHERE event_id = $1",
        )
        .bind(event_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0)
    }

    pub async fn get_photo_count(&self, event_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM photos WHERE event_id = $1",
        )
        .bind(event_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0)
    }

    pub async fn get_review_count(&self, event_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM event_reviews WHERE event_id = $1",
        )
        .bind(event_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0)
    }

    pub async fn is_member(&self, event_id: Uuid, user_id: Uuid) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM event_members WHERE event_id = $1 AND user_id = $2",
        )
        .bind(event_id)
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0 > 0)
    }
}
