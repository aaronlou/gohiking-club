use sqlx::PgPool;
use uuid::Uuid;

use crate::models::agent::{AgentConversation, AgentMemory, AgentMessage, AgentSkill};

pub struct AgentRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AgentRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    // ── Conversations ──

    pub async fn create_conversation(&self, user_id: Uuid, title: Option<&str>) -> anyhow::Result<AgentConversation> {
        let convo = sqlx::query_as::<_, AgentConversation>(
            "INSERT INTO agent_conversations (user_id, title) VALUES ($1, $2) RETURNING *",
        )
        .bind(user_id)
        .bind(title)
        .fetch_one(self.pool)
        .await?;
        Ok(convo)
    }

    pub async fn list_conversations(&self, user_id: Uuid) -> anyhow::Result<Vec<AgentConversation>> {
        let convos = sqlx::query_as::<_, AgentConversation>(
            "SELECT * FROM agent_conversations WHERE user_id = $1 ORDER BY updated_at DESC LIMIT 50",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        Ok(convos)
    }

    pub async fn find_conversation(&self, id: Uuid) -> anyhow::Result<Option<AgentConversation>> {
        let convo = sqlx::query_as::<_, AgentConversation>(
            "SELECT * FROM agent_conversations WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(convo)
    }

    pub async fn touch_conversation(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE agent_conversations SET updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_conversation(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM agent_conversations WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    // ── Messages ──

    pub async fn create_message(
        &self,
        conversation_id: Uuid,
        role: &str,
        content: &str,
        tool_calls: Option<&serde_json::Value>,
        tool_call_id: Option<&str>,
    ) -> anyhow::Result<AgentMessage> {
        let msg = sqlx::query_as::<_, AgentMessage>(
            "INSERT INTO agent_messages (conversation_id, role, content, tool_calls, tool_call_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(conversation_id)
        .bind(role)
        .bind(content)
        .bind(tool_calls)
        .bind(tool_call_id)
        .fetch_one(self.pool)
        .await?;
        Ok(msg)
    }

    pub async fn list_messages(&self, conversation_id: Uuid, limit: usize) -> anyhow::Result<Vec<AgentMessage>> {
        let msgs = sqlx::query_as::<_, AgentMessage>(
            "SELECT * FROM agent_messages WHERE conversation_id = $1 ORDER BY created_at ASC LIMIT $2",
        )
        .bind(conversation_id)
        .bind(limit as i64)
        .fetch_all(self.pool)
        .await?;
        Ok(msgs)
    }

    pub async fn count_messages(&self, conversation_id: Uuid) -> anyhow::Result<i64> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM agent_messages WHERE conversation_id = $1",
        )
        .bind(conversation_id)
        .fetch_one(self.pool)
        .await?;
        Ok(count)
    }

    // ── Memories ──

    pub async fn upsert_memory(&self, user_id: Uuid, key: &str, value: &str) -> anyhow::Result<AgentMemory> {
        let mem = sqlx::query_as::<_, AgentMemory>(
            r#"
            INSERT INTO agent_memories (user_id, key, value) VALUES ($1, $2, $3)
            ON CONFLICT (user_id, key) DO UPDATE SET value = $3, updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(key)
        .bind(value)
        .fetch_one(self.pool)
        .await?;
        Ok(mem)
    }

    pub async fn get_memory(&self, user_id: Uuid, key: &str) -> anyhow::Result<Option<AgentMemory>> {
        let mem = sqlx::query_as::<_, AgentMemory>(
            "SELECT * FROM agent_memories WHERE user_id = $1 AND key = $2",
        )
        .bind(user_id)
        .bind(key)
        .fetch_optional(self.pool)
        .await?;
        Ok(mem)
    }

    pub async fn list_memories(&self, user_id: Uuid) -> anyhow::Result<Vec<AgentMemory>> {
        let mems = sqlx::query_as::<_, AgentMemory>(
            "SELECT * FROM agent_memories WHERE user_id = $1 ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        Ok(mems)
    }

    pub async fn delete_memory(&self, user_id: Uuid, key: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM agent_memories WHERE user_id = $1 AND key = $2")
            .bind(user_id)
            .bind(key)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    // ── Skills ──

    pub async fn upsert_skill(&self, name: &str, version: &str, source: &str) -> anyhow::Result<AgentSkill> {
        let skill = sqlx::query_as::<_, AgentSkill>(
            r#"
            INSERT INTO agent_skills (name, version, source) VALUES ($1, $2, $3)
            ON CONFLICT (name) DO UPDATE SET version = $2, source = $3
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(version)
        .bind(source)
        .fetch_one(self.pool)
        .await?;
        Ok(skill)
    }

    pub async fn list_skills(&self) -> anyhow::Result<Vec<AgentSkill>> {
        let skills = sqlx::query_as::<_, AgentSkill>(
            "SELECT * FROM agent_skills ORDER BY name ASC",
        )
        .fetch_all(self.pool)
        .await?;
        Ok(skills)
    }

    pub async fn delete_skill(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM agent_skills WHERE name = $1")
            .bind(name)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
