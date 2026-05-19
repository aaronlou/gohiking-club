use async_trait::async_trait;
use sqlx::PgPool;

use crate::agent::{Tool, ToolDefinition};
use crate::models::event::Event;

pub struct ListEventsTool {
    pool: PgPool,
}

impl ListEventsTool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Tool for ListEventsTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "list_events".into(),
            description: "List upcoming and recent hiking events. Returns event details including title, location, date, and status.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "description": "Filter by event status: upcoming, ongoing, past, or all"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of events to return (default 5)"
                    }
                },
                "required": []
            }),
        }
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<String> {
        let status = args["status"].as_str().unwrap_or("all");
        let limit = args["limit"].as_i64().unwrap_or(5).clamp(1, 20);

        let events = if status == "all" {
            sqlx::query_as::<_, Event>(
                "SELECT * FROM events ORDER BY date DESC LIMIT $1",
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Event>(
                "SELECT * FROM events WHERE status::text = $1 ORDER BY date DESC LIMIT $2",
            )
            .bind(status)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };

        if events.is_empty() {
            Ok("No events found.".to_string())
        } else {
            let results: Vec<serde_json::Value> = events
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "id": e.id,
                        "title": e.title,
                        "description": e.description,
                        "location": e.location,
                        "date": e.date,
                        "status": e.status,
                    })
                })
                .collect();
            Ok(serde_json::to_string_pretty(&results)?)
        }
    }
}
