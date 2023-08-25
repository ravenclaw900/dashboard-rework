use crate::sysdata::Request;
use crate::types::SystemData;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use color_eyre::{eyre::WrapErr, Report};
use tokio::sync::{mpsc, oneshot};

pub async fn system(State(tx): State<mpsc::Sender<Request>>) -> Result<Json<SystemData>, ApiError> {
    let (sysdata_tx, sysdata_rx) = oneshot::channel();
    tx.send(Request::System(sysdata_tx))
        .await
        .wrap_err("failed to send cpu request")?;

    let sysdata = sysdata_rx.await.wrap_err("failed to receive system data")?;

    Ok(Json(sysdata))
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
