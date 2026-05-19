use async_trait::async_trait;
use serde_json::Value;

use crate::agent::{ChatMessage, LlmProvider, StreamChunk, ToolCall, ToolDefinition};

pub struct ClaudeLlm {
    api_key: String,
    model: String,
    max_tokens: u32,
    client: reqwest::Client,
}

impl ClaudeLlm {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-sonnet-4-20250514".into(),
            max_tokens: 2048,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    fn build_messages(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> Value {
        let system = messages.iter().find(|m| m.role == "system");

        let mut chat_msgs: Vec<Value> = Vec::new();
        for m in messages.iter().filter(|m| m.role != "system") {
            let mut content: Vec<Value> = vec![serde_json::json!({"type": "text", "text": m.content})];

            if let Some(tc) = &m.tool_calls {
                for tc in tc {
                    content.push(serde_json::json!({
                        "type": "tool_use",
                        "id": tc.id,
                        "name": tc.name,
                        "input": tc.arguments,
                    }));
                }
            }

            let mut msg = serde_json::json!({
                "role": if m.role == "assistant" { "assistant" } else { "user" },
                "content": content,
            });

            if m.role == "tool" {
                msg = serde_json::json!({
                    "role": "user",
                    "content": [{
                        "type": "tool_result",
                        "tool_use_id": m.tool_call_id,
                        "content": m.content,
                    }],
                });
            }

            chat_msgs.push(msg);
        }

        let anthropic_tools: Vec<Value> = tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters,
                })
            })
            .collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": chat_msgs,
        });

        if let Some(sys) = system {
            body["system"] = serde_json::json!(sys.content);
        }

        if !anthropic_tools.is_empty() {
            body["tools"] = serde_json::json!(anthropic_tools);
        }

        body
    }
}

#[async_trait]
impl LlmProvider for ClaudeLlm {
    fn name(&self) -> &'static str {
        "claude"
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<ChatMessage> {
        let mut body = self.build_messages(messages, tools);
        body["stream"] = serde_json::json!(false);

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;

        let content = &data["content"];
        let mut text = String::new();
        let mut tool_calls: Vec<ToolCall> = Vec::new();

        if let Some(items) = content.as_array() {
            for item in items {
                match item["type"].as_str() {
                    Some("text") => {
                        if let Some(t) = item["text"].as_str() {
                            text.push_str(t);
                        }
                    }
                    Some("tool_use") => {
                        tool_calls.push(ToolCall {
                            id: item["id"].as_str().unwrap_or("").to_string(),
                            name: item["name"].as_str().unwrap_or("").to_string(),
                            arguments: item["input"].clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(ChatMessage {
            role: "assistant".into(),
            content: text,
            tool_calls: if tool_calls.is_empty() {
                None
            } else {
                Some(tool_calls)
            },
            tool_call_id: None,
        })
    }

    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
        sender: tokio::sync::mpsc::UnboundedSender<StreamChunk>,
    ) -> anyhow::Result<()> {
        let mut body = self.build_messages(messages, tools);
        body["stream"] = serde_json::json!(true);

        let resp = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();
        let mut current_tool_id = String::new();
        let mut current_tool_name = String::new();
        let mut current_tool_args = String::new();

        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find("\n\n") {
                let event_str = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                for line in event_str.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(event) = serde_json::from_str::<Value>(data) {
                            match event["type"].as_str() {
                                Some("content_block_start") => {
                                    if event["content_block"]["type"] == "tool_use" {
                                        current_tool_id = event["content_block"]["id"]
                                            .as_str()
                                            .unwrap_or("")
                                            .to_string();
                                        current_tool_name = event["content_block"]["name"]
                                            .as_str()
                                            .unwrap_or("")
                                            .to_string();
                                        current_tool_args = String::new();
                                    }
                                }
                                Some("content_block_delta") => {
                                    let delta = &event["delta"];
                                    match delta["type"].as_str() {
                                        Some("text_delta") => {
                                            let _ = sender.send(StreamChunk {
                                                delta: delta["text"]
                                                    .as_str()
                                                    .unwrap_or("")
                                                    .to_string(),
                                                tool_calls: None,
                                                finish_reason: None,
                                            });
                                        }
                                        Some("input_json_delta") => {
                                            if let Some(s) = delta["partial_json"].as_str() {
                                                current_tool_args.push_str(s);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                Some("content_block_stop") => {
                                    if !current_tool_id.is_empty() {
                                        let args = serde_json::from_str(&current_tool_args)
                                            .unwrap_or(serde_json::Value::Null);
                                        let _ = sender.send(StreamChunk {
                                            delta: String::new(),
                                            tool_calls: Some(vec![ToolCall {
                                                id: current_tool_id.clone(),
                                                name: current_tool_name.clone(),
                                                arguments: args,
                                            }]),
                                            finish_reason: None,
                                        });
                                        current_tool_id.clear();
                                        current_tool_name.clear();
                                        current_tool_args.clear();
                                    }
                                }
                                Some("message_stop") => {
                                    let _ = sender.send(StreamChunk {
                                        delta: String::new(),
                                        tool_calls: None,
                                        finish_reason: Some("stop".into()),
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
