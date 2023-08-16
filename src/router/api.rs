use crate::sysdata::SystemRequest;
use crate::types::System;
use axum::{extract::State, http::StatusCode, Json};
use tokio::sync::{mpsc, oneshot};

pub async fn system(
    State(tx): State<mpsc::Sender<SystemRequest>>,
) -> Result<Json<System>, StatusCode> {
    let (sys_tx, sys_rx) = oneshot::channel();
    tx.send(SystemRequest::Cpu(sys_tx))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cpu = sys_rx
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(System { cpu }))
}
