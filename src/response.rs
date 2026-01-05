use axum::{
    extract::FromRequest,
    response::{IntoResponse, Response},
};

use crate::error::ServiceError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ServiceError))]
pub struct ResponseJson<T>(pub T);

impl<T> IntoResponse for ResponseJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
