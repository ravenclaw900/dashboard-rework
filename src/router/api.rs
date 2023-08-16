use crate::sysdata::Request;
use crate::types::System;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use color_eyre::{eyre::WrapErr, Report};
use tokio::sync::{mpsc, oneshot};

pub async fn system(State(tx): State<mpsc::Sender<Request>>) -> Result<Json<System>, ApiError> {
    let (cpu_tx, cpu_rx) = oneshot::channel();
    tx.send(Request::Cpu(cpu_tx))
        .await
        .wrap_err("failed to send cpu request")?;

    let (mem_tx, mem_rx) = oneshot::channel();
    tx.send(Request::Memory(mem_tx))
        .await
        .wrap_err("failed to send memory request")?;

    let cpu = cpu_rx.await.wrap_err("failed to receive cpu value")?;
    let mem = mem_rx.await.wrap_err("failed to receive memory value")?;

    Ok(Json(System {
        cpu,
        ram: mem.ram,
        swap: mem.swap,
    }))
}

pub struct ApiError(Report);

impl From<Report> for ApiError {
    fn from(value: Report) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("{}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
