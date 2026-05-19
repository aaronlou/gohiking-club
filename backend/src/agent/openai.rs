use async_trait::async_trait;
use serde_json::Value;

use crate::agent::{ChatMessage, LlmProvider, StreamChunk, ToolCall, ToolDefinition};

pub struct OpenAILlm {
    api_key: String,
    model: String,
    max_tokens: u32,
    client: reqwest::Client,
}

impl OpenAILlm {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4o".into(),
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

    fn build_openai_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            })
            .collect()
    }

    fn build_openai_messages(messages: &[ChatMessage]) -> Vec<Value> {
        messages
            .iter()
            .map(|m| {
                let mut msg = serde_json::json!({
                    "role": m.role,
                    "content": m.content,
                });

                if let Some(tc) = &m.tool_calls {
                    msg["tool_calls"] = serde_json::json!(tc
                        .iter()
                        .map(|t| serde_json::json!({
                            "id": t.id,
                            "type": "function",
                            "function": {
                                "name": t.name,
                                "arguments": serde_json::to_string(&t.arguments).unwrap_or_default(),
                            }
                        }))
                        .collect::<Vec<_>>());
                }

                if let Some(tcid) = &m.tool_call_id {
                    msg["role"] = serde_json::json!("tool");
                    msg["tool_call_id"] = serde_json::json!(tcid);
                }

                msg
            })
            .collect()
    }
}

#[async_trait]
impl LlmProvider for OpenAILlm {
    fn name(&self) -> &'static str {
        "openai"
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<ChatMessage> {
        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": Self::build_openai_messages(messages),
        });

        let openai_tools = Self::build_openai_tools(tools);
        if !openai_tools.is_empty() {
            body["tools"] = serde_json::json!(openai_tools);
        }

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        let choice = &data["choices"][0]["message"];

        let text = choice["content"].as_str().unwrap_or("").to_string();
        let tool_calls = if let Some(tcs) = choice["tool_calls"].as_array() {
            let calls: Vec<ToolCall> = tcs
                .iter()
                .map(|tc| {
                    let args: serde_json::Value = serde_json::from_str(
                        tc["function"]["arguments"].as_str().unwrap_or("{}"),
                    )
                    .unwrap_or(serde_json::Value::Null);
                    ToolCall {
                        id: tc["id"].as_str().unwrap_or("").to_string(),
                        name: tc["function"]["name"].as_str().unwrap_or("").to_string(),
                        arguments: args,
                    }
                })
                .collect();
            if calls.is_empty() {
                None
            } else {
                Some(calls)
            }
        } else {
            None
        };

        Ok(ChatMessage {
            role: "assistant".into(),
            content: text,
            tool_calls,
            tool_call_id: None,
        })
    }

    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
        sender: tokio::sync::mpsc::UnboundedSender<StreamChunk>,
    ) -> anyhow::Result<()> {
        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": Self::build_openai_messages(messages),
            "stream": true,
        });

        let openai_tools = Self::build_openai_tools(tools);
        if !openai_tools.is_empty() {
            body["tools"] = serde_json::json!(openai_tools);
        }

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();

        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        let _ = sender.send(StreamChunk {
                            delta: String::new(),
                            tool_calls: None,
                            finish_reason: Some("stop".into()),
                        });
                        continue;
                    }

                    if let Ok(event) = serde_json::from_str::<Value>(data) {
                        let choice = &event["choices"][0];
                        let delta = &choice["delta"];

                        if let Some(content) = delta["content"].as_str() {
                            let _ = sender.send(StreamChunk {
                                delta: content.to_string(),
                                tool_calls: None,
                                finish_reason: None,
                            });
                        }

                        if let Some(tcs) = delta["tool_calls"].as_array() {
                            let calls: Vec<ToolCall> = tcs
                                .iter()
                                .filter_map(|tc| {
                                    let func = &tc["function"];
                                    Some(ToolCall {
                                        id: tc["id"].as_str()?.to_string(),
                                        name: func["name"].as_str()?.to_string(),
                                        arguments: serde_json::from_str(
                                            func["arguments"].as_str().unwrap_or("{}"),
                                        )
                                        .unwrap_or(serde_json::Value::Null),
                                    })
                                })
                                .collect();
                            if !calls.is_empty() {
                                let _ = sender.send(StreamChunk {
                                    delta: String::new(),
                                    tool_calls: Some(calls),
                                    finish_reason: None,
                                });
                            }
                        }

                        if let Some(reason) = choice["finish_reason"].as_str() {
                            if !reason.is_empty() {
                                let _ = sender.send(StreamChunk {
                                    delta: String::new(),
                                    tool_calls: None,
                                    finish_reason: Some(reason.to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
