use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn exists_by_username(&self, username: &str) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0 > 0)
    }

    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_photo_count(&self, user_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM photos WHERE user_id = $1 AND status = 'approved'",
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0)
    }
}
