use super::api;
use crate::sysdata;
use axum::{routing::get, Router};

fn api_router() -> Router {
    let tx = sysdata::system_task();
    Router::new().route("/cpu", get(api::system)).with_state(tx)
}

pub fn router() -> Router {
    Router::new().nest("/api", api_router())
}
