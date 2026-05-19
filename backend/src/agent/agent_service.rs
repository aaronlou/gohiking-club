use sqlx::PgPool;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::agent::memory::MemoryService;
use crate::agent::skills::SkillLoader;
use crate::agent::tools::ToolRegistry;
use crate::agent::{ChatMessage, LlmRegistry, StreamChunk, ToolCall, ToolDefinition};

/// Orchestrates the agent: LLM + tools + memory + skills.
pub struct AgentService {
    pool: PgPool,
    memory: MemoryService,
    llm: LlmRegistry,
    tools: ToolRegistry,
    skills: SkillLoader,
    system_prompt: String,
}

impl AgentService {
    pub fn new(
        pool: PgPool,
        llm: LlmRegistry,
        tools: ToolRegistry,
        skills: SkillLoader,
        system_prompt: String,
        max_history_messages: usize,
    ) -> Self {
        Self {
            memory: MemoryService::new(pool.clone(), max_history_messages),
            pool,
            llm,
            tools,
            skills,
            system_prompt,
        }
    }

    /// Build the full message list for the LLM.
    async fn build_messages(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        user_message: &str,
    ) -> anyhow::Result<Vec<ChatMessage>> {
        let memories = self.memory.load_memories_context(user_id).await?;

        let matching_skills = self.skills.find_matching(user_message);
        let skills_text = if matching_skills.is_empty() {
            String::new()
        } else {
            let mut s = String::from("\n## Available Skills\n");
            for skill in matching_skills {
                s.push_str(&format!(
                    "### {}\n{}\n{}\n",
                    skill.name, skill.description, skill.body
                ));
            }
            s
        };

        let system_content = format!("{}\n{}\n{}", self.system_prompt, memories, skills_text);

        let mut messages = vec![ChatMessage {
            role: "system".into(),
            content: system_content,
            tool_calls: None,
            tool_call_id: None,
        }];

        let history = self.memory.load_history(conversation_id).await?;
        messages.extend(history);

        messages.push(ChatMessage {
            role: "user".into(),
            content: user_message.to_string(),
            tool_calls: None,
            tool_call_id: None,
        });

        Ok(messages)
    }

    /// Non-streaming chat: user message → agent response.
    pub async fn chat(
        &self,
        user_id: Uuid,
        conversation_id: Option<Uuid>,
        user_message: &str,
    ) -> anyhow::Result<(Uuid, String)> {
        let convo_id = self
            .memory
            .save_user_message(conversation_id, user_id, user_message)
            .await?;

        let messages = self.build_messages(user_id, convo_id, user_message).await?;
        let tool_defs = self.tools.get_definitions();

        let (content, tool_calls) = self.run_agent_loop(&messages, &tool_defs, convo_id).await?;

        self.memory
            .save_assistant_message(
                convo_id,
                &content,
                tool_calls
                    .as_ref()
                    .map(|tc| serde_json::to_value(tc).unwrap_or_default())
                    .as_ref(),
            )
            .await?;

        Ok((convo_id, content))
    }

    /// Streaming chat: user message → SSE stream of agent response.
    pub async fn chat_stream(
        &self,
        user_id: Uuid,
        conversation_id: Option<Uuid>,
        user_message: &str,
        sender: UnboundedSender<StreamChunk>,
    ) -> anyhow::Result<Uuid> {
        let convo_id = self
            .memory
            .save_user_message(conversation_id, user_id, user_message)
            .await?;

        let messages = self.build_messages(user_id, convo_id, user_message).await?;
        let tool_defs = self.tools.get_definitions();
        let llm = self.llm.active();

        // Stream first LLM response
        let (inner_tx, mut inner_rx) = tokio::sync::mpsc::unbounded_channel();

        let llm2 = llm.clone();
        let msgs = messages.clone();
        let tds = tool_defs.clone();
        let stream_task = tokio::spawn(async move {
            llm2.chat_stream(&msgs, &tds, inner_tx).await
        });

        let mut full_content = String::new();
        let mut full_tool_calls: Vec<ToolCall> = Vec::new();

        while let Some(chunk) = inner_rx.recv().await {
            if let Some(ref tcs) = chunk.tool_calls {
                for tc in tcs {
                    full_tool_calls.push(tc.clone());
                }
                let _ = sender.send(chunk);
            } else if chunk.finish_reason.is_some() {
                // Will handle after the loop
            } else {
                if !chunk.delta.is_empty() {
                    full_content.push_str(&chunk.delta);
                    let _ = sender.send(chunk);
                }
            }
        }

        let _ = stream_task.await;

        // Handle tool calls if any
        if !full_tool_calls.is_empty() {
            let tc_json = serde_json::to_value(&full_tool_calls).unwrap_or_default();
            self.memory
                .save_assistant_message(convo_id, &full_content, Some(&tc_json))
                .await?;

            let mut continued_msgs = messages.clone();
            continued_msgs.push(ChatMessage {
                role: "assistant".into(),
                content: full_content.clone(),
                tool_calls: Some(full_tool_calls.clone()),
                tool_call_id: None,
            });

            for tc in &full_tool_calls {
                match self.tools.execute(&tc.name, tc.arguments.clone()).await {
                    Ok(result) => {
                        let _ = sender.send(StreamChunk {
                            delta: String::new(),
                            tool_calls: None,
                            finish_reason: None,
                        });

                        self.memory
                            .save_tool_result(convo_id, &tc.id, &result)
                            .await?;

                        continued_msgs.push(ChatMessage {
                            role: "tool".into(),
                            content: result,
                            tool_calls: None,
                            tool_call_id: Some(tc.id.clone()),
                        });
                    }
                    Err(e) => {
                        tracing::warn!("Tool {} failed: {}", tc.name, e);
                    }
                }
            }

            // Continue streaming with tool results
            let (inner_tx2, mut inner_rx2) = tokio::sync::mpsc::unbounded_channel();
            let llm3 = llm.clone();
            let tds2 = tool_defs.clone();
            let cont_task = tokio::spawn(async move {
                llm3.chat_stream(&continued_msgs, &tds2, inner_tx2).await
            });

            let mut continuation_text = String::new();
            while let Some(chunk) = inner_rx2.recv().await {
                if chunk.finish_reason.is_none() && !chunk.delta.is_empty() {
                    continuation_text.push_str(&chunk.delta);
                    let _ = sender.send(chunk);
                }
            }

            let _ = cont_task.await;

            if !continuation_text.is_empty() {
                self.memory
                    .save_assistant_message(convo_id, &continuation_text, None)
                    .await?;
            }
        } else {
            self.memory
                .save_assistant_message(convo_id, &full_content, None)
                .await?;
        }

        let _ = sender.send(StreamChunk {
            delta: String::new(),
            tool_calls: None,
            finish_reason: Some("done".into()),
        });

        Ok(convo_id)
    }

    /// Agent loop for non-streaming: call LLM, execute tools, repeat.
    async fn run_agent_loop(
        &self,
        messages: &[ChatMessage],
        tool_defs: &[ToolDefinition],
        convo_id: Uuid,
    ) -> anyhow::Result<(String, Option<Vec<ToolCall>>)> {
        let mut current_msgs = messages.to_vec();
        let llm = self.llm.active();

        loop {
            let response = llm.chat(&current_msgs, tool_defs).await?;

            match response.tool_calls {
                Some(ref tool_calls) if !tool_calls.is_empty() => {
                    current_msgs.push(ChatMessage {
                        role: "assistant".into(),
                        content: response.content.clone(),
                        tool_calls: Some(tool_calls.clone()),
                        tool_call_id: None,
                    });

                    for tc in tool_calls {
                        match self.tools.execute(&tc.name, tc.arguments.clone()).await {
                            Ok(result) => {
                                self.memory
                                    .save_tool_result(convo_id, &tc.id, &result)
                                    .await?;
                                current_msgs.push(ChatMessage {
                                    role: "tool".into(),
                                    content: result,
                                    tool_calls: None,
                                    tool_call_id: Some(tc.id.clone()),
                                });
                            }
                            Err(e) => {
                                tracing::warn!("Tool {} failed: {}", tc.name, e);
                                current_msgs.push(ChatMessage {
                                    role: "tool".into(),
                                    content: format!("Error: {}", e),
                                    tool_calls: None,
                                    tool_call_id: Some(tc.id.clone()),
                                });
                            }
                        }
                    }
                }
                _ => {
                    return Ok((response.content, response.tool_calls));
                }
            }
        }
    }

    pub fn skills(&self) -> &SkillLoader {
        &self.skills
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
