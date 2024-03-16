use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::Method,
    middleware::{self, Next},
    response::Response,
    Router,
};
use minify::html::minify;
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
        .layer(middleware::from_fn(minify_html_response))
        .layer(compression_layer)
        .layer(size_limit_layer)
        .layer(cors_layer)
        .layer(tracing_layer)
}

async fn minify_html_response(request: Request, next: Next) -> Response<Body> {
    let response = next.run(request).await;

    let Some(content_type) = response.headers().get("content-type") else {
        return response;
    };

    if !content_type.to_str().unwrap().contains("text/html") {
        return response;
    }

    let (mut parts, body) = response.into_parts();

    return axum::body::to_bytes(body, usize::MAX)
        .await
        .map(|b| minify(String::from_utf8_lossy(&b).as_ref()))
        .map(|b| {
            let content_length = b.len();

            parts
                .headers
                .insert("content-length", content_length.into());

            Response::from_parts(parts, Body::from(b))
        })
        .unwrap();
}
