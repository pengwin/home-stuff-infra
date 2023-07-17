use axum::http::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateCorsLayerError {
    #[error("InvalidOrigin: {0:?}")]
    InvalidOrigin(#[from] InvalidHeaderValue),
}
