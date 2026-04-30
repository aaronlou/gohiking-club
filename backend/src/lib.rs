pub mod api;
pub mod models;
pub mod services;
pub mod ai;
pub mod infra;
pub mod config;

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::ai::registry::ScorerRegistry;
use crate::ai::{claude::ClaudeScorer, ollama::OllamaScorer, openai::OpenAIScorer, PhotoScorer};
use crate::config::AppConfig;
use crate::infra::db;
use crate::infra::storage::Storage;
use crate::services::auth_service::AuthService;
use crate::services::photo_service::PhotoService;
use crate::services::scoring_service::ScoringService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub photo_service: Arc<PhotoService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    fn new(pool: PgPool, photo_service: PhotoService, auth_service: AuthService) -> Self {
        Self {
            pool,
            photo_service: Arc::new(photo_service),
            auth_service: Arc::new(auth_service),
        }
    }
}

pub async fn build_app(config: AppConfig) -> anyhow::Result<Router> {
    let pool = db::init_pool(&config.database.url).await?;

    // Initialize S3 storage
    let storage = Storage::new(
        config.storage.endpoint.clone(),
        config.storage.region.clone(),
        config.storage.bucket.clone(),
    )
    .await?;

    // Build AI scorer registry
    let providers = build_scorers(&config);
    let registry = ScorerRegistry::new(providers);
    let scorer_service = ScoringService::new(
        registry,
        config.ai_scoring.threshold,
        config.ai_scoring.concurrent_limit,
    );

    let photo_service = PhotoService::new(pool.clone(), storage, scorer_service);
    let auth_service = AuthService::new(&config.auth);
    let state = AppState::new(pool, photo_service, auth_service);

    let app = Router::new()
        // Auth
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/auth/me", get(api::auth::me))
        // Photos
        .route("/api/photos", post(api::photos::upload).get(api::photos::list))
        .route("/api/photos/:id", get(api::photos::get).delete(api::photos::delete))
        // Users
        .route("/api/users/:id", get(api::users::get_profile))
        // Events
        .route("/api/events", post(api::events::create).get(api::events::list))
        .route("/api/events/:id", get(api::events::get))
        .route("/api/events/:id/join", post(api::events::join))
        .route("/api/events/:id/photos", get(api::events::get_photos))
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
            _ => {
                tracing::warn!("Unknown scorer provider: {name}, skipping");
                continue;
            }
        };
        providers.insert(name.clone(), scorer);
    }

    providers
}
