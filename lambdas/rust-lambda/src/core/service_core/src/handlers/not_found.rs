use axum::{response::IntoResponse, response::Response};

use crate::responses::ErrorResponse;

pub async fn handler_404() -> Response {
    ErrorResponse::not_found("Page not found").into_response()
}
