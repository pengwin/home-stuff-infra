use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct ErrorResponse {
    error: String,
    #[serde(skip)]
    code: StatusCode,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, error: &str) -> Self {
        Self {
            code,
            error: error.to_owned(),
        }
    }

    pub fn not_found(error: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, error)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.code, Json(self)).into_response()
    }
}
