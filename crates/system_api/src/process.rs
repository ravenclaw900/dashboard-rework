use axum::extract::{Path, Query, State};
use serde::Deserialize;
use sysdata::{Request, RequestTx};

#[derive(Deserialize)]
pub struct ProcessSignalQuery {
    signal: sysdata::types::ProcessSignal,
}

pub async fn process_signal(
    State(tx): State<RequestTx>,
    Path(pid): Path<usize>,
    signal: Query<ProcessSignalQuery>,
) {
    tx.send(Request::ProcessSignal(pid, signal.0.signal))
        .await
        .unwrap();
}
