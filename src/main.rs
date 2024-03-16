mod routes;
mod telemetry;
mod templating;

const ENV_FILTER: &str = "htmx-todo=debug,tower_http=debug";

#[tokio::main]
async fn main() {
    telemetry::setup_telemetry(ENV_FILTER);

    let router = routes::router();

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {addr}");
    axum::serve(listener, router).await.unwrap();
}
