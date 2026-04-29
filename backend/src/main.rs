use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = gohiking_backend::config::AppConfig::load()?;
    let app = gohiking_backend::build_app(config.clone()).await?;

    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
