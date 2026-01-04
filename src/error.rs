use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrResponse {
    pub code: &'static str,
    pub message: String,
}

impl ErrResponse {
    pub fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for ErrResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Not found: {msg}")]
    NotFound { msg: String },
    #[error("route does not exist")]
    RouteNotFound,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();

        let (status_code, code) = match self {
            Self::NotFound { msg: _ } => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
            Self::RouteNotFound => (StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND"),
        };

        (status_code, ErrResponse::new(code, message)).into_response()
    }
}
