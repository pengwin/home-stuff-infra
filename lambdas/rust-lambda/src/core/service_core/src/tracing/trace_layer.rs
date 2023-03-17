use std::time::Duration;

use axum::http::Request;

use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse, TraceLayer};
use tracing::{info, Span};

#[derive(Clone)]
pub struct TraceSpan {
    parent: Span,
}

impl TraceSpan {
    pub fn new(parent: Span) -> Self {
        Self { parent }
    }
}

impl<B> MakeSpan<B> for TraceSpan {
    fn make_span(&mut self, _request: &Request<B>) -> Span {
        tracing::info_span!(parent: &self.parent, "tracing_layer")
    }
}

#[derive(Clone, Default)]
pub struct TraceRequest;

impl<B> OnRequest<B> for TraceRequest {
    fn on_request(&mut self, request: &Request<B>, span: &Span) {
        let method = request.method();
        span.record("method", &tracing::field::display(method));
        let uri = request.uri();
        span.record("uri", &tracing::field::display(uri));
        info!("{} {}", method, uri)
    }
}

#[derive(Clone, Default)]
pub struct TraceResponse {}

impl<B> OnResponse<B> for TraceResponse {
    fn on_response(self, response: &axum::http::Response<B>, latency: Duration, span: &Span) {
        let status = response.status();
        span.record("status_code", &tracing::field::display(status));
        info!("{} {:?}", status, latency)
    }
}

pub fn create_layer(
    parent: Span,
) -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, TraceSpan, TraceRequest, TraceResponse> {
    TraceLayer::new_for_http()
        .make_span_with(TraceSpan::new(parent))
        .on_request(TraceRequest::default())
        .on_response(TraceResponse::default())
}
