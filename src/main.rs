mod routes;

#[tokio::main]
async fn main() {
    let router = routes::router();

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {addr}");
    axum::serve(listener, router).await.unwrap();
}
