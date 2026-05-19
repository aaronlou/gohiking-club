use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Database row structs ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentConversation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
    pub tool_call_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentMemory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentSkill {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub source: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

// ── Request DTOs ──

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub conversation_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct InstallSkillRequest {
    pub name: String,
}

// ── Response DTOs ──

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub conversation_id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: i64,
}

impl From<AgentConversation> for ConversationResponse {
    fn from(c: AgentConversation) -> Self {
        Self {
            id: c.id,
            title: c.title,
            created_at: c.created_at,
            updated_at: c.updated_at,
            message_count: 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl From<AgentMessage> for MessageResponse {
    fn from(m: AgentMessage) -> Self {
        Self {
            id: m.id,
            role: m.role,
            content: m.content,
            tool_calls: m.tool_calls,
            created_at: m.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SkillResponse {
    pub name: String,
    pub description: String,
    pub version: String,
    pub source: String,
    pub triggers: Vec<String>,
    pub enabled: bool,
}
