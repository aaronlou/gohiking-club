use sqlx::PgPool;
use uuid::Uuid;

use crate::agent::ChatMessage;
use crate::repositories::agent_repository::AgentRepository;

/// Manages conversation history and semantic memory for agent conversations.
pub struct MemoryService {
    pool: PgPool,
    max_history_messages: usize,
}

impl MemoryService {
    pub fn new(pool: PgPool, max_history_messages: usize) -> Self {
        Self {
            pool,
            max_history_messages,
        }
    }

    /// Load conversation messages as ChatMessage for the LLM.
    /// Returns them in chronological order, limited to max_history_messages.
    pub async fn load_history(&self, conversation_id: Uuid) -> anyhow::Result<Vec<ChatMessage>> {
        let repo = AgentRepository::new(&self.pool);
        let messages = repo
            .list_messages(conversation_id, self.max_history_messages)
            .await?;

        let chat_msgs: Vec<ChatMessage> = messages
            .into_iter()
            .map(|m| ChatMessage {
                role: m.role,
                content: m.content,
                tool_calls: m.tool_calls.map(|v| {
                    serde_json::from_value(v).unwrap_or_default()
                }),
                tool_call_id: m.tool_call_id,
            })
            .collect();

        Ok(chat_msgs)
    }

    /// Save a user message to the conversation.
    /// Returns the conversation_id (creates new if needed).
    pub async fn save_user_message(
        &self,
        conversation_id: Option<Uuid>,
        user_id: Uuid,
        content: &str,
    ) -> anyhow::Result<Uuid> {
        let repo = AgentRepository::new(&self.pool);

        let convo_id = match conversation_id {
            Some(id) => {
                repo.touch_conversation(id).await?;
                id
            }
            None => {
                let title = if content.len() > 80 {
                    format!("{}...", &content[..80])
                } else {
                    content.to_string()
                };
                let convo = repo.create_conversation(user_id, Some(&title)).await?;
                convo.id
            }
        };

        repo.create_message(convo_id, "user", content, None, None)
            .await?;

        Ok(convo_id)
    }

    /// Save an assistant response message to the conversation.
    pub async fn save_assistant_message(
        &self,
        conversation_id: Uuid,
        content: &str,
        tool_calls: Option<&serde_json::Value>,
    ) -> anyhow::Result<()> {
        let repo = AgentRepository::new(&self.pool);
        repo.create_message(conversation_id, "assistant", content, tool_calls, None)
            .await?;
        repo.touch_conversation(conversation_id).await?;
        Ok(())
    }

    /// Save a tool result message to the conversation.
    pub async fn save_tool_result(
        &self,
        conversation_id: Uuid,
        tool_call_id: &str,
        result: &str,
    ) -> anyhow::Result<()> {
        let repo = AgentRepository::new(&self.pool);
        repo.create_message(conversation_id, "tool", result, None, Some(tool_call_id))
            .await?;
        Ok(())
    }

    /// Store a semantic memory for a user.
    pub async fn remember(&self, user_id: Uuid, key: &str, value: &str) -> anyhow::Result<()> {
        let repo = AgentRepository::new(&self.pool);
        repo.upsert_memory(user_id, key, value).await?;
        Ok(())
    }

    /// Retrieve a specific memory for a user.
    pub async fn recall(&self, user_id: Uuid, key: &str) -> anyhow::Result<Option<String>> {
        let repo = AgentRepository::new(&self.pool);
        let mem = repo.get_memory(user_id, key).await?;
        Ok(mem.map(|m| m.value))
    }

    /// Load all memories for a user, formatted as context for the system prompt.
    pub async fn load_memories_context(&self, user_id: Uuid) -> anyhow::Result<String> {
        let repo = AgentRepository::new(&self.pool);
        let memories = repo.list_memories(user_id).await?;

        if memories.is_empty() {
            return Ok(String::new());
        }

        let lines: Vec<String> = memories
            .iter()
            .map(|m| format!("- {}: {}", m.key, m.value))
            .collect();

        Ok(format!(
            "## User Preferences (from past conversations)\n{}\n",
            lines.join("\n")
        ))
    }
}
