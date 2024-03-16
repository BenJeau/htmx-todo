use axum::response::sse::Event;
use tokio::sync::{broadcast::Sender, mpsc::UnboundedReceiver};

use crate::state::{IdCounter, Todos};

enum EventKind {
    Total,
    Done,
    CurrentIdx,
}

impl EventKind {
    fn name(&self) -> &'static str {
        match self {
            Self::Total => "total",
            Self::Done => "done",
            Self::CurrentIdx => "current_idx",
        }
    }

    fn event(&self, data: usize) -> Event {
        Event::default().event(self.name()).data(data.to_string())
    }
}

pub fn event_relayer(
    todos: Todos,
    id_counter: IdCounter,
    sse_tx: Sender<Event>,
    mut stats_rx: UnboundedReceiver<()>,
) {
    tokio::spawn(async move {
        loop {
            stats_rx.recv().await;

            let todos_data = todos.read().await;

            let total_event = EventKind::Total.event(todos_data.len());
            let _ = sse_tx.send(total_event);

            let done_event =
                EventKind::Done.event(todos_data.iter().filter(|todo| todo.done).count());
            let _ = sse_tx.send(done_event);

            let counter = id_counter.read().await;

            let current_idx_event = EventKind::CurrentIdx.event(*counter + 1);
            let _ = sse_tx.send(current_idx_event);
        }
    });
}
