use axum::{http::StatusCode, response::IntoResponse, response::Response, routing::get, Router};
use utoipa::Modify;

const HEALTHCHECK_ENDPOINT: &str = "/healthcheck";

pub struct HealthCheckAddon;

impl Modify for HealthCheckAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let path = utoipa::openapi::PathItem::new(
            utoipa::openapi::PathItemType::Get,
            utoipa::openapi::path::OperationBuilder::new()
                .responses(
                    utoipa::openapi::ResponsesBuilder::new()
                        .response("200", utoipa::openapi::Response::new("Healthy")),
                )
                .response("500", utoipa::openapi::Response::new("Unhealthy"))
                .operation_id(Some("healthcheck"))
                .tag("healthcheck"),
        );
        openapi
            .paths
            .paths
            .insert(HEALTHCHECK_ENDPOINT.to_string(), path);
    }
}

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = 200, description = "Pet found successfully", body = Pet),
        (status = 404, description = "Pet was not found")
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
async fn healthcheck_handler() -> Response {
    (StatusCode::OK, "Healthy").into_response()
}

pub fn healthcheck() -> Router {
    Router::new().route(HEALTHCHECK_ENDPOINT, get(healthcheck_handler))
}
