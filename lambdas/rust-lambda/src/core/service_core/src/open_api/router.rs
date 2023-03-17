use utoipa::{openapi::ServerBuilder, OpenApi};

use axum::{routing::get, Router};

pub fn router<T: OpenApi>(endpoint_url: &str) -> Router {
    let server_builder = ServerBuilder::new().url(endpoint_url);
    let servers = vec![server_builder.build()];

    let path = "/api-doc/openapi.json";
    let mut doc = T::openapi();
    doc.servers = Some(servers);
    match doc.to_pretty_json() {
        Ok(open_api) => Router::new().route(path, get(|| async { open_api })),
        Err(e) => {
            let res = format!("Unable to build open api spec: {}", e);
            Router::new().route(path, get(|| async { res }))
        }
    }
}
