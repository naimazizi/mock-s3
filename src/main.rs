use axum::{
    Router,
    extract::FromRequest,
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::{
    error::ServiceError,
    service::file_discovery::{list_all_files, serve_file_discovery},
};

pub mod config;
pub mod error;
pub mod service;

#[derive(Clone)]
pub struct AppState {
    pub env: config::Config,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ServiceError))]
pub struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
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
        .route("/list-assets", get(list_all_files))
        .with_state(state.clone())
        .merge(serve_file_discovery(state.clone()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
