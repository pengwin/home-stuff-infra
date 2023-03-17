use axum::{
    extract::rejection::{ExtensionRejection, TypedHeaderRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use thiserror::Error;

use super::AuthErrorResponse;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("UuidError: {0:?}")]
    UuidError(#[from] uuid::Error),
    #[error("WrongCredentials")]
    WrongCredentials,
    #[error("WrongRole")]
    WrongRole,
    #[error("InvalidTokenHeader: {0:?}")]
    InvalidTokenHeader(#[from] TypedHeaderRejection),
    #[error("InvalidTokenExtension: {0:?}")]
    InvalidTokenExtension(#[from] ExtensionRejection),
    #[error("PersistenceError: {0:?}")]
    PersistenceError(String),
    #[error("JwtGenerateError: {0:?}")]
    JwtGenerateError(jsonwebtoken::errors::Error),
    #[error("JwtParseError: {0:?}")]
    JwtParseError(jsonwebtoken::errors::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::UuidError(ue) => (
                StatusCode::BAD_REQUEST,
                Json(AuthErrorResponse {
                    error: format!("UUID error: {:?}", ue),
                }),
            )
                .into_response(),
            AuthError::PersistenceError(pe) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthErrorResponse {
                    error: format!("SDK error: {:?}", pe),
                }),
            )
                .into_response(),
            AuthError::JwtGenerateError(je) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthErrorResponse {
                    error: format!("Generate JWT error: {:?}", je),
                }),
            )
                .into_response(),
            AuthError::JwtParseError(je) => (
                StatusCode::UNAUTHORIZED,
                Json(AuthErrorResponse {
                    error: format!("JWT error: {:?}", je),
                }),
            )
                .into_response(),
            AuthError::InvalidTokenHeader(ie) => (
                StatusCode::UNAUTHORIZED,
                Json(AuthErrorResponse {
                    error: format!("InvalidTokenHeader {:?}", ie),
                }),
            )
                .into_response(),
            AuthError::InvalidTokenExtension(ie) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthErrorResponse {
                    error: format!("InvalidTokenExtension {:?}", ie),
                }),
            )
                .into_response(),
            AuthError::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(AuthErrorResponse {
                    error: "Wrong credentials".to_string(),
                }),
            )
                .into_response(),
            AuthError::WrongRole => (
                StatusCode::FORBIDDEN,
                Json(AuthErrorResponse {
                    error: "WrongRole".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
