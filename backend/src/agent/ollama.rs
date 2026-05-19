use async_trait::async_trait;
use serde_json::Value;

use crate::agent::{ChatMessage, LlmProvider, StreamChunk, ToolCall, ToolDefinition};

pub struct OllamaLlm {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaLlm {
    pub fn new(endpoint: String, model: String) -> Self {
        Self {
            endpoint,
            model,
            client: reqwest::Client::new(),
        }
    }

    fn build_ollama_format(messages: &[ChatMessage]) -> Vec<Value> {
        messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content,
                })
            })
            .collect()
    }

    fn build_ollama_tools(tools: &[ToolDefinition]) -> Vec<Value> {
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
}

#[async_trait]
impl LlmProvider for OllamaLlm {
    fn name(&self) -> &'static str {
        "ollama"
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<ChatMessage> {
        let mut body = serde_json::json!({
            "model": self.model,
            "messages": Self::build_ollama_format(messages),
            "stream": false,
        });

        let ollama_tools = Self::build_ollama_tools(tools);
        if !ollama_tools.is_empty() {
            body["tools"] = serde_json::json!(ollama_tools);
        }

        let resp = self
            .client
            .post(format!("{}/api/chat", self.endpoint.trim_end_matches('/')))
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        let msg = &data["message"];

        let text = msg["content"].as_str().unwrap_or("").to_string();
        let tool_calls = if let Some(tcs) = msg["tool_calls"].as_array() {
            let calls: Vec<ToolCall> = tcs
                .iter()
                .enumerate()
                .map(|(i, tc)| ToolCall {
                    id: format!("call_{i}"),
                    name: tc["function"]["name"].as_str().unwrap_or("").to_string(),
                    arguments: tc["function"]["arguments"].clone(),
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
            "messages": Self::build_ollama_format(messages),
            "stream": true,
        });

        let ollama_tools = Self::build_ollama_tools(tools);
        if !ollama_tools.is_empty() {
            body["tools"] = serde_json::json!(ollama_tools);
        }

        let resp = self
            .client
            .post(format!("{}/api/chat", self.endpoint.trim_end_matches('/')))
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

                if let Ok(event) = serde_json::from_str::<Value>(&line) {
                    if let Some(content) = event["message"]["content"].as_str() {
                        let _ = sender.send(StreamChunk {
                            delta: content.to_string(),
                            tool_calls: None,
                            finish_reason: None,
                        });
                    }

                    if event["done"].as_bool() == Some(true) {
                        let _ = sender.send(StreamChunk {
                            delta: String::new(),
                            tool_calls: None,
                            finish_reason: Some("stop".into()),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}
