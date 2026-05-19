use async_trait::async_trait;
use serde_json::Value;

use crate::agent::{ChatMessage, LlmProvider, StreamChunk, ToolCall, ToolDefinition};

pub struct GeminiLlm {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiLlm {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gemini-2.0-flash".into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    fn build_gemini_contents(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> (Vec<Value>, Vec<Value>, Vec<Value>) {
        let system_instruction = messages.iter().find(|m| m.role == "system");

        let mut contents: Vec<Value> = Vec::new();
        for m in messages.iter().filter(|m| m.role != "system") {
            let role = match m.role.as_str() {
                "assistant" => "model",
                "tool" => "function",
                _ => "user",
            };

            let mut parts: Vec<Value> = Vec::new();

            if !m.content.is_empty() {
                parts.push(serde_json::json!({"text": m.content}));
            }

            if let Some(tc) = &m.tool_calls {
                for t in tc {
                    parts.push(serde_json::json!({
                        "functionCall": {
                            "name": t.name,
                            "args": t.arguments,
                        }
                    }));
                }
            }

            if role == "function" {
                parts.push(serde_json::json!({
                    "functionResponse": {
                        "name": m.tool_call_id.as_deref().unwrap_or(""),
                        "response": {"result": m.content},
                    }
                }));
            }

            contents.push(serde_json::json!({
                "role": role,
                "parts": parts,
            }));
        }

        let gemini_tools: Vec<Value> = if tools.is_empty() {
            Vec::new()
        } else {
            vec![serde_json::json!({
                "functionDeclarations": tools.iter().map(|t| {
                    serde_json::json!({
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    })
                }).collect::<Vec<_>>(),
            })]
        };

        let sys: Vec<Value> = system_instruction
            .map(|s| {
                vec![serde_json::json!({
                    "parts": [{"text": s.content.clone()}]
                })]
            })
            .unwrap_or_default();

        (contents, gemini_tools, sys)
    }
}

#[async_trait]
impl LlmProvider for GeminiLlm {
    fn name(&self) -> &'static str {
        "gemini"
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<ChatMessage> {
        let (contents, gemini_tools, system_instruction) = self.build_gemini_contents(messages, tools);

        let mut body = serde_json::json!({
            "contents": contents,
        });

        if !gemini_tools.is_empty() {
            body["tools"] = serde_json::json!(gemini_tools);
        }

        if !system_instruction.is_empty() {
            body["system_instruction"] = system_instruction[0].clone();
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let resp = self.client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error: {}", err_text);
        }

        let data: Value = resp.json().await?;
        let candidate = &data["candidates"][0]["content"];

        let mut text = String::new();
        let mut tool_calls: Vec<ToolCall> = Vec::new();

        if let Some(parts) = candidate["parts"].as_array() {
            for part in parts {
                if let Some(t) = part["text"].as_str() {
                    text.push_str(t);
                }
                if let Some(fc) = part.get("functionCall") {
                    tool_calls.push(ToolCall {
                        id: format!("call_{}", tool_calls.len()),
                        name: fc["name"].as_str().unwrap_or("").to_string(),
                        arguments: fc["args"].clone(),
                    });
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
        let (contents, gemini_tools, system_instruction) = self.build_gemini_contents(messages, tools);

        let mut body = serde_json::json!({
            "contents": contents,
        });

        if !gemini_tools.is_empty() {
            body["tools"] = serde_json::json!(gemini_tools);
        }

        if !system_instruction.is_empty() {
            body["system_instruction"] = system_instruction[0].clone();
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            self.model, self.api_key
        );

        let resp = self.client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error: {}", err_text);
        }

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();

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
                            if let Some(candidates) = event["candidates"].as_array() {
                                for candidate in candidates {
                                    if let Some(parts) = candidate["content"]["parts"].as_array() {
                                        for part in parts {
                                            if let Some(t) = part["text"].as_str() {
                                                let _ = sender.send(StreamChunk {
                                                    delta: t.to_string(),
                                                    tool_calls: None,
                                                    finish_reason: None,
                                                });
                                            }
                                        }
                                    }

                                    if let Some(reason) = candidate["finishReason"].as_str() {
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
            }
        }

        Ok(())
    }
}
