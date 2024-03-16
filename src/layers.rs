use axum::{
    extract::{DefaultBodyLimit, Request},
    http::Method,
    Router,
};
use tower_http::{
    compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, Predicate,
    },
    cors::CorsLayer,
    trace::TraceLayer,
    CompressionLevel,
};

pub fn apply_layers(router: Router) -> Router {
    let cors_layer = CorsLayer::new().allow_methods(vec![
        Method::GET,
        Method::POST,
        Method::DELETE,
        Method::PUT,
    ]);

    let compression_layer = CompressionLayer::new()
        .quality(CompressionLevel::Fastest)
        .compress_when(SizeAbove::new(1064).and(NotForContentType::new("text/event-stream")));

    let tracing_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
        )
    });

    let size_limit_layer = DefaultBodyLimit::max(1024 * 1024);

    router
        .layer(compression_layer)
        .layer(size_limit_layer)
        .layer(cors_layer)
        .layer(tracing_layer)
}
