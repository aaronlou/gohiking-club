pub mod agent;
pub mod api;
pub mod filters;
pub mod models;
pub mod repositories;
pub mod services;
pub mod ai;
pub mod infra;
pub mod config;

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::agent::agent_service::AgentService;
use crate::agent::skills::SkillLoader;
use crate::agent::tools::ToolRegistry;
use crate::agent::LlmProvider;
use crate::agent::LlmRegistry;
use crate::ai::registry::ScorerRegistry;
use crate::ai::{claude::ClaudeScorer, gemini::GeminiScorer, ollama::OllamaScorer, openai::OpenAIScorer, PhotoScorer};
use crate::config::AppConfig;
use crate::infra::db;
use crate::infra::local_storage::LocalStorage;
use crate::infra::storage::S3Storage;
use crate::infra::storage_backend::StorageBackend;
use crate::infra::veimagex::VeImageXStorage;
use crate::services::auth_service::AuthService;
use crate::services::photo_service::PhotoService;
use crate::services::scoring_service::ScoringService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub photo_service: Arc<PhotoService>,
    pub auth_service: Arc<AuthService>,
    pub agent_service: Arc<AgentService>,
}

impl AppState {
    fn new(
        pool: PgPool,
        photo_service: PhotoService,
        auth_service: AuthService,
        agent_service: AgentService,
    ) -> Self {
        Self {
            pool,
            photo_service: Arc::new(photo_service),
            auth_service: Arc::new(auth_service),
            agent_service: Arc::new(agent_service),
        }
    }
}

pub async fn build_app(config: AppConfig) -> anyhow::Result<Router> {
    let pool = db::init_pool(&config.database.url).await?;

    // Initialize storage backend
    let storage: Arc<dyn StorageBackend> = match config.storage.backend.as_str() {
        "veimagex" => {
            let vc = config.storage.veimagex.clone().expect("veImageX config required when backend = \"veimagex\"");
            Arc::new(VeImageXStorage::new(
                vc.service_id,
                vc.domain,
                vc.access_key,
                vc.secret_key,
            ))
        }
        "local" => {
            let local = config.storage.local.clone().expect("Local config required when backend = \"local\"");
            Arc::new(LocalStorage::new(&local.base_dir, &local.public_url_prefix)?)
        }
        _ => {
            let s3 = config.storage.s3.clone().expect("S3 config required when backend = \"s3\"");
            Arc::new(
                S3Storage::new(
                    s3.endpoint,
                    s3.region,
                    s3.bucket,
                    s3.public_endpoint,
                )
                .await?,
            )
        }
    };

    // Build AI scorer registry
    let providers = build_scorers(&config);
    let registry = ScorerRegistry::new(providers, &config.ai_scoring.active);
    let scorer_service = ScoringService::new(
        registry,
        config.ai_scoring.threshold,
        config.ai_scoring.concurrent_limit,
    );

    let photo_service = PhotoService::new(pool.clone(), storage, scorer_service);
    let auth_service = AuthService::new(&config.auth);

    // Build Agent
    let llm_providers = build_llm_providers(&config);
    let llm_registry = LlmRegistry::new(llm_providers, &config.agent.active);

    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(Box::new(
        crate::agent::tools::search_photos::SearchPhotosTool::new(pool.clone()),
    ));
    tool_registry.register(Box::new(
        crate::agent::tools::list_events::ListEventsTool::new(pool.clone()),
    ));
    tool_registry.register(Box::new(
        crate::agent::tools::get_user_profile::GetUserProfileTool::new(pool.clone()),
    ));

    let skill_loader = SkillLoader::new(&config.agent.skills)?;

    let agent_service = AgentService::new(
        pool.clone(),
        llm_registry,
        tool_registry,
        skill_loader,
        config.agent.system_prompt.clone(),
        config.agent.max_history_messages,
    );

    let state = AppState::new(pool, photo_service, auth_service, agent_service);

    let app = Router::new()
        // Static uploads (for local storage backend)
        .nest_service("/uploads", ServeDir::new("./uploads"))
        // Auth
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/auth/me", get(api::auth::me))
        // Photos
        .route("/api/photos", post(api::photos::upload).layer(DefaultBodyLimit::max(20 * 1024 * 1024)).get(api::photos::list))
        .route("/api/photos/:id", get(api::photos::get).delete(api::photos::delete))
        // Users
        .route("/api/users/:id", get(api::users::get_profile))
        // Events
        .route("/api/events", post(api::events::create).get(api::events::list))
        .route("/api/events/:id", get(api::events::get))
        .route("/api/events/:id/join", post(api::events::join))
        .route("/api/events/:id/photos", get(api::events::get_photos))
        // Teams
        .route("/api/teams", post(api::teams::create).get(api::teams::list))
        .route("/api/teams/:id", get(api::teams::get).put(api::teams::update))
        .route("/api/teams/:id/join", post(api::teams::join))
        .route("/api/teams/:id/leave", post(api::teams::leave))
        .route("/api/teams/:id/members", get(api::teams::get_members))
        .route("/api/teams/:id/members/:user_id/role", put(api::teams::update_member_role))
        .route("/api/teams/:id/events", get(api::teams::get_events))
        // Team invitations
        .route("/api/teams/:id/invitations", post(api::teams::create_invitation).get(api::teams::list_invitations))
        .route("/api/teams/invitations/:code", get(api::teams::get_invitation_by_code))
        .route("/api/teams/invitations/:code/apply", post(api::teams::apply_join))
        // Team join requests
        .route("/api/teams/:id/join-requests", get(api::teams::list_join_requests))
        .route("/api/teams/:id/join-requests/approve", post(api::teams::approve_join_request))
        .route("/api/teams/:id/join-requests/reject", post(api::teams::reject_join_request))
        // Event Reviews
        .route("/api/events/:id/reviews", post(api::event_reviews::create).get(api::event_reviews::list))
        .route("/api/events/:id/reviews/:review_id", post(api::event_reviews::delete))
        // Agent
        .route("/api/agent/chat", post(api::agent::chat))
        .route("/api/agent/conversations", get(api::agent::list_conversations))
        .route("/api/agent/conversations/:id", get(api::agent::get_conversation).delete(api::agent::delete_conversation))
        .route("/api/agent/skills", get(api::agent::list_skills))
        .route("/api/agent/skills/install", post(api::agent::install_skill))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app)
}

fn build_scorers(config: &AppConfig) -> HashMap<String, Box<dyn PhotoScorer>> {
    let mut providers: HashMap<String, Box<dyn PhotoScorer>> = HashMap::new();

    for (name, provider_config) in &config.ai_scoring.providers {
        let scorer: Box<dyn PhotoScorer> = match name.as_str() {
            "claude" => {
                let api_key = std::env::var("CLAUDE_API_KEY").unwrap_or_default();
                Box::new(
                    ClaudeScorer::new(api_key)
                        .with_model(&provider_config.model)
                        .with_max_tokens(provider_config.max_tokens.unwrap_or(500)),
                )
            }
            "openai" => {
                let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
                Box::new(OpenAIScorer::new(api_key).with_model(&provider_config.model))
            }
            "ollama" => {
                let endpoint = provider_config
                    .endpoint
                    .clone()
                    .unwrap_or_else(|| "http://localhost:11434".into());
                Box::new(OllamaScorer::new(endpoint, provider_config.model.clone()))
            }
            "gemini" => {
                let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
                Box::new(GeminiScorer::new(api_key).with_model(&provider_config.model))
            }
            _ => {
                tracing::warn!("Unknown scorer provider: {name}, skipping");
                continue;
            }
        };
        providers.insert(name.clone(), scorer);
    }

    providers
}

fn build_llm_providers(config: &AppConfig) -> HashMap<String, Arc<dyn LlmProvider>> {
    let mut providers: HashMap<String, Arc<dyn LlmProvider>> = HashMap::new();

    for (name, provider_config) in &config.agent.providers {
        let llm: Arc<dyn LlmProvider> = match name.as_str() {
            "claude" => {
                let api_key = std::env::var("CLAUDE_API_KEY").unwrap_or_default();
                Arc::new(
                    crate::agent::claude::ClaudeLlm::new(api_key)
                        .with_model(&provider_config.model)
                        .with_max_tokens(provider_config.max_tokens.unwrap_or(2048)),
                )
            }
            "openai" => {
                let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
                Arc::new(
                    crate::agent::openai::OpenAILlm::new(api_key)
                        .with_model(&provider_config.model)
                        .with_max_tokens(provider_config.max_tokens.unwrap_or(2048)),
                )
            }
            "gemini" => {
                let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
                Arc::new(
                    crate::agent::gemini::GeminiLlm::new(api_key)
                        .with_model(&provider_config.model),
                )
            }
            "ollama" => {
                let endpoint = provider_config
                    .endpoint
                    .clone()
                    .unwrap_or_else(|| "http://localhost:11434".into());
                Arc::new(crate::agent::ollama::OllamaLlm::new(
                    endpoint,
                    provider_config.model.clone(),
                ))
            }
            _ => {
                tracing::warn!("Unknown LLM provider: {name}, skipping");
                continue;
            }
        };
        providers.insert(name.clone(), llm);
    }

    providers
}
