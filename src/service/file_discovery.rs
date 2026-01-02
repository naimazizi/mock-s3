use axum::{Router, handler::HandlerWithoutStateExt, http::StatusCode};
use tower_http::services::ServeDir;

use crate::AppState;

pub fn serve_file_discovery(state: AppState) -> Router {
    let dir = state.env.asset_dir;
    let service_404 = handle_404_asset.into_service();

    let serve_dir_assets = ServeDir::new(dir).not_found_service(service_404);
    Router::new().nest_service("/assets", serve_dir_assets)
}

async fn handle_404_asset() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
