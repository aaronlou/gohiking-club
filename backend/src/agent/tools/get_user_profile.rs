use async_trait::async_trait;
use sqlx::PgPool;

use crate::agent::{Tool, ToolDefinition};
use crate::models::user::User;

pub struct GetUserProfileTool {
    pool: PgPool,
}

impl GetUserProfileTool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Tool for GetUserProfileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "get_user_profile".into(),
            description: "Get the profile information of a user by username or user ID. Returns username, bio, and photo count.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "username": {
                        "type": "string",
                        "description": "Username to look up"
                    }
                },
                "required": ["username"]
            }),
        }
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<String> {
        let username = args["username"].as_str().unwrap_or("");

        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match user {
            Some(u) => {
                let result = serde_json::json!({
                    "id": u.id,
                    "username": u.username,
                    "bio": u.bio,
                    "created_at": u.created_at,
                });
                Ok(serde_json::to_string_pretty(&result)?)
            }
            None => Ok(format!("User '{}' not found.", username)),
        }
    }
}
