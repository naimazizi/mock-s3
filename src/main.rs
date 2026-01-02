use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub mod config;
pub mod service;

#[derive(Clone)]
pub struct AppState {
    pub env: config::Config,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let config = config::Config::init();
    let host_port = format!("{}:{}", &config.app_host, &config.app_port);

    let state = AppState {
        env: config.clone(),
    };

    let app = Router::new()
        .route("/", get(root))
        .with_state(state.clone())
        .merge(service::file_discovery::serve_file_discovery(state.clone()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();
    let _ = axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
