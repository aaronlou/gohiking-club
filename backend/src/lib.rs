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
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

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
            // Default to S3 / MinIO
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
        // Static uploads (for local storage backend)
        .nest_service("/uploads", ServeDir::new("./uploads"))
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
        // Teams
        .route("/api/teams", post(api::teams::create).get(api::teams::list))
        .route("/api/teams/:id", get(api::teams::get).put(api::teams::update))
        .route("/api/teams/:id/join", post(api::teams::join))
        .route("/api/teams/:id/leave", post(api::teams::leave))
        .route("/api/teams/:id/members", get(api::teams::get_members))
        .route("/api/teams/:id/events", get(api::teams::get_events))
        // Event Reviews
        .route("/api/events/:id/reviews", post(api::event_reviews::create).get(api::event_reviews::list))
        .route("/api/events/:id/reviews/:review_id", post(api::event_reviews::delete))
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
