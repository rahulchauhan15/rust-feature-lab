mod app_state;
mod config;
mod handlers;
mod router;
mod services;
use anyhow::{Context, Result};
use app_state::AppState;
use config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new("cargo_feature_mock_poc=debug")
                }),
        )
        .init();

    //  Config 
    let config = AppConfig::from_env()
        .context("failed to load application config")?;

    tracing::info!(port = config.port, "config loaded");

    let weather = services::build_weather_service(&config.weather_api_key);
    let s3 = services::s3_service::build_s3_service(&config.s3_bucket);
    //  State 
    let state = AppState { weather, s3 };

    //  Router 
    let app = router::build_router(state);

    //  Listener 
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind TCP listener on {addr}"))?;

    tracing::info!(addr, "server listening — press Ctrl-C to stop");

    //  Serve 
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server encountered a fatal error")?;

    tracing::info!("server stopped cleanly");
    Ok(())
}

/// Resolves on SIGINT (Ctrl-C) or SIGTERM, triggering graceful shutdown.
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c    => tracing::info!("received Ctrl-C"),
        () = terminate => tracing::info!("received SIGTERM"),
    }
}
