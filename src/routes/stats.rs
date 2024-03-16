use axum::{
    extract::State,
    response::{sse::KeepAlive, IntoResponse, Sse},
};
use tokio_stream::wrappers::BroadcastStream;

use crate::state::AppState;

pub async fn sse_stats(State(state): State<AppState>) -> impl IntoResponse {
    let rx = state.sse_tx.subscribe();

    Sse::new(BroadcastStream::new(rx))
        .keep_alive(KeepAlive::default())
        .into_response()
}
