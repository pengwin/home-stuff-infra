use axum::http::{HeaderValue, Method};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;

use super::error::CreateCorsLayerError;

pub fn create_cors_layer(origin: &str) -> Result<CorsLayer, CreateCorsLayerError> {
    let origin = origin.parse::<HeaderValue>()?;

    let layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        // allow requests from any origin
        .allow_origin(origin)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    Ok(layer)
}
