mod layers;
mod routes;
mod sse;
mod state;
mod telemetry;
mod templating;

const ENV_FILTER: &str = "htmx_todo=debug,tower_http=debug";

#[tokio::main]
async fn main() {
    telemetry::setup_telemetry(ENV_FILTER);

    let (stats_tx, stats_rx) = tokio::sync::mpsc::unbounded_channel();
    let (sse_tx, _) = tokio::sync::broadcast::channel(100);

    let state = state::AppState::new(stats_tx, sse_tx.clone());

    sse::event_relayer(
        state.todos.clone(),
        state.id_counter.clone(),
        state.sse_tx.clone(),
        stats_rx,
    );

    let router = routes::router(state);
    let app_with_layers = layers::apply_layers(router);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {addr}");
    axum::serve(listener, app_with_layers)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
