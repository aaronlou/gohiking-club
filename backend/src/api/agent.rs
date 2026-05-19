use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use std::convert::Infallible;
use futures::stream::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::auth_extractor::AuthenticatedUser;
use crate::AppState;
use crate::models::agent::{
    ChatRequest, ConversationResponse, InstallSkillRequest, MessageResponse, SkillResponse,
};
use crate::repositories::agent_repository::AgentRepository;

/// POST /api/agent/chat — SSE streaming chat
pub async fn chat(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::unbounded_channel();

    let service = state.agent_service.clone();
    let user_id = auth_user.id;
    let message = body.message.clone();
    let conversation_id = body.conversation_id;

    tokio::spawn(async move {
        match service
            .chat_stream(user_id, conversation_id, &message, tx)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Agent chat error: {}", e);
            }
        }
    });

    let stream = UnboundedReceiverStream::new(rx).map(|chunk| {
        if chunk.finish_reason.as_deref() == Some("done") {
            Ok(Event::default()
                .event("done")
                .data("{}"))
        } else if chunk.tool_calls.is_some() {
            Ok(Event::default()
                .event("tool_call")
                .data(serde_json::to_string(&chunk).unwrap_or_default()))
        } else {
            Ok(Event::default()
                .event("delta")
                .data(serde_json::to_string(&chunk).unwrap_or_default()))
        }
    });

    Sse::new(stream)
}

/// GET /api/agent/conversations — list user's conversations
pub async fn list_conversations(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<ConversationResponse>>, (axum::http::StatusCode, String)> {
    let repo = AgentRepository::new(&state.pool);
    let convos = repo
        .list_conversations(auth_user.id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list conversations: {e}"),
            )
        })?;

    let mut responses: Vec<ConversationResponse> = convos.into_iter().map(ConversationResponse::from).collect();

    for resp in &mut responses {
        match repo.count_messages(resp.id).await {
            Ok(c) => resp.message_count = c,
            Err(_) => {}
        }
    }

    Ok(Json(responses))
}

/// GET /api/agent/conversations/:id — get conversation messages
pub async fn get_conversation(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<Vec<MessageResponse>>, (axum::http::StatusCode, String)> {
    let repo = AgentRepository::new(&state.pool);

    let convo = repo.find_conversation(id).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to find conversation: {e}"),
        )
    })?;

    match convo {
        Some(c) if c.user_id == auth_user.id => {}
        Some(_) => {
            return Err((axum::http::StatusCode::FORBIDDEN, "Access denied".into()));
        }
        None => {
            return Err((axum::http::StatusCode::NOT_FOUND, "Conversation not found".into()));
        }
    }

    let messages = repo.list_messages(id, 200).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to list messages: {e}"),
        )
    })?;

    Ok(Json(messages.into_iter().map(MessageResponse::from).collect()))
}

/// DELETE /api/agent/conversations/:id
pub async fn delete_conversation(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<(), (axum::http::StatusCode, String)> {
    let repo = AgentRepository::new(&state.pool);

    let convo = repo.find_conversation(id).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to find conversation: {e}"),
        )
    })?;

    match convo {
        Some(c) if c.user_id == auth_user.id => {}
        Some(_) => {
            return Err((axum::http::StatusCode::FORBIDDEN, "Access denied".into()));
        }
        None => {
            return Err((axum::http::StatusCode::NOT_FOUND, "Conversation not found".into()));
        }
    }

    repo.delete_conversation(id).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete conversation: {e}"),
        )
    })
}

/// GET /api/agent/skills — list loaded skills
pub async fn list_skills(
    State(state): State<AppState>,
) -> Json<Vec<SkillResponse>> {
    let skills = state.agent_service.skills().list();
    let responses: Vec<SkillResponse> = skills
        .iter()
        .map(|s| SkillResponse {
            name: s.name.clone(),
            description: s.description.clone(),
            version: s.version.clone(),
            source: format!("{:?}", s.source).to_lowercase(),
            triggers: s.triggers.clone(),
            enabled: true,
        })
        .collect();

    Json(responses)
}

/// POST /api/agent/skills/install — install a skill from ClawHub
pub async fn install_skill(
    _auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(body): Json<InstallSkillRequest>,
) -> Result<Json<SkillResponse>, (axum::http::StatusCode, String)> {
    // Use clawhub CLI to install the skill
    let output = tokio::process::Command::new("npx")
        .args([
            "clawhub@latest",
            "install",
            &body.name,
            "--dir",
            "./skills",
        ])
        .output()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to run clawhub: {e}"),
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            format!("ClawHub install failed: {}", stderr),
        ));
    }

    // Record in database
    let repo = AgentRepository::new(&state.pool);
    repo.upsert_skill(&body.name, "unknown", "clawhub")
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to record skill: {e}"),
            )
        })?;

    Ok(Json(SkillResponse {
        name: body.name,
        description: String::new(),
        version: "unknown".into(),
        source: "clawhub".into(),
        triggers: vec![],
        enabled: true,
    }))
}
