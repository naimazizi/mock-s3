use axum::{Router, extract::State, handler::HandlerWithoutStateExt, http::StatusCode};
use tokio::fs;
use tower_http::services::ServeDir;

use crate::{AppJson, AppState, error::ServiceError};

pub fn serve_file_discovery(state: AppState) -> Router {
    let dir = state.env.asset_dir;
    let service_404 = handle_404_asset.into_service();

    let serve_dir_assets = ServeDir::new(dir).not_found_service(service_404);
    Router::new().nest_service("/assets", serve_dir_assets)
}

pub async fn list_all_files(
    State(state): State<AppState>,
) -> Result<AppJson<Vec<String>>, ServiceError> {
    let dir = state.env.asset_dir;

    let entries = fs::read_dir(dir).await;
    let mut result: Vec<String> = Vec::new();

    match entries {
        Ok(mut entry) => {
            while let Some(entry) = entry.next_entry().await.unwrap() {
                result.push(entry.file_name().into_string().unwrap())
            }
        }
        Err(_e) => {}
    }

    if result.is_empty() {
        return Err(ServiceError::NotFound {
            msg: "No assets found".to_string(),
        });
    }

    Ok(AppJson(result))
}

async fn handle_404_asset() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
