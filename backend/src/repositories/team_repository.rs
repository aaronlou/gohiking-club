use sqlx::PgPool;
use uuid::Uuid;

use crate::models::team::{Team, TeamMember};

pub struct TeamRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TeamRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
        created_by: Uuid,
    ) -> anyhow::Result<Team> {
        let team = sqlx::query_as::<_, Team>(
            r#"
            INSERT INTO teams (name, slug, description, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(created_by)
        .fetch_one(self.pool)
        .await?;
        Ok(team)
    }

    /// Create team and add creator as admin in a single transaction.
    pub async fn create_with_creator(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
        created_by: Uuid,
    ) -> anyhow::Result<Team> {
        let mut tx = self.pool.begin().await?;

        let team = sqlx::query_as::<_, Team>(
            r#"
            INSERT INTO teams (name, slug, description, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(created_by)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO team_members (team_id, user_id, role) VALUES ($1, $2, 'admin')",
        )
        .bind(team.id)
        .bind(created_by)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(team)
    }

    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Team>> {
        let team = sqlx::query_as::<_, Team>("SELECT * FROM teams WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;
        Ok(team)
    }

    pub async fn find_by_slug(&self, slug: &str) -> anyhow::Result<Option<Team>> {
        let team = sqlx::query_as::<_, Team>("SELECT * FROM teams WHERE slug = $1")
            .bind(slug)
            .fetch_optional(self.pool)
            .await?;
        Ok(team)
    }

    pub async fn exists_by_slug(&self, slug: &str) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM teams WHERE slug = $1")
            .bind(slug)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0 > 0)
    }

    pub async fn list(
        &self,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Team>> {
        let teams = sqlx::query_as::<_, Team>(
            r#"
            SELECT * FROM teams
            WHERE ($1::text IS NULL OR status = $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;
        Ok(teams)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        logo_url: Option<&str>,
        cover_url: Option<&str>,
    ) -> anyhow::Result<Team> {
        let team = sqlx::query_as::<_, Team>(
            r#"
            UPDATE teams
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                logo_url = COALESCE($4, logo_url),
                cover_url = COALESCE($5, cover_url),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(logo_url)
        .bind(cover_url)
        .fetch_one(self.pool)
        .await?;
        Ok(team)
    }

    pub async fn add_member(&self, team_id: Uuid, user_id: Uuid, role: &str) -> anyhow::Result<bool> {
        let result = sqlx::query(
            "INSERT INTO team_members (team_id, user_id, role) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        )
        .bind(team_id)
        .bind(user_id)
        .bind(role)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_member(&self, team_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id)
            .bind(user_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_members(&self, team_id: Uuid) -> anyhow::Result<Vec<TeamMember>> {
        let members = sqlx::query_as::<_, TeamMember>(
            r#"
            SELECT tm.team_id, tm.user_id, tm.role, tm.joined_at
            FROM team_members tm
            JOIN users u ON tm.user_id = u.id
            WHERE tm.team_id = $1
            ORDER BY tm.joined_at ASC
            "#,
        )
        .bind(team_id)
        .fetch_all(self.pool)
        .await?;
        Ok(members)
    }

    pub async fn is_admin(&self, team_id: Uuid, user_id: Uuid) -> anyhow::Result<bool> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM team_members WHERE team_id = $1 AND user_id = $2 AND role = 'admin'",
        )
        .bind(team_id)
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count.0 > 0)
    }

    pub async fn get_member_count(&self, team_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM team_members WHERE team_id = $1")
            .bind(team_id)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0)
    }

    pub async fn get_event_count(&self, team_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE team_id = $1")
            .bind(team_id)
            .fetch_one(self.pool)
            .await?;
        Ok(count.0)
    }
}
