use async_trait::async_trait;
use sqlx::PgPool;

use crate::agent::{Tool, ToolDefinition};
use crate::models::photo::Photo;

pub struct SearchPhotosTool {
    pool: PgPool,
}

impl SearchPhotosTool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Tool for SearchPhotosTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "search_photos".into(),
            description: "Search photos by keyword in title and description. Returns matching photos with URLs, scores, and metadata.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search keyword to match against photo titles and descriptions"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<String> {
        let query = args["query"].as_str().unwrap_or("");
        let photos = sqlx::query_as::<_, Photo>(
            "SELECT * FROM photos WHERE title ILIKE $1 OR description ILIKE $1 ORDER BY ai_score DESC LIMIT 10",
        )
        .bind(format!("%{query}%"))
        .fetch_all(&self.pool)
        .await?;

        if photos.is_empty() {
            Ok(format!("No photos found matching '{}'.", query))
        } else {
            let results: Vec<serde_json::Value> = photos
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "id": p.id,
                        "title": p.title,
                        "description": p.description,
                        "url": p.url,
                        "ai_score": p.ai_score,
                        "status": p.status,
                    })
                })
                .collect();
            Ok(serde_json::to_string_pretty(&results)?)
        }
    }
}
