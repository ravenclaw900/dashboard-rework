use super::{home, vendored};
use crate::sysdata;

use axum::routing::get;
use axum::Router;

fn vendored_router() -> Router {
    Router::new().route("/htmx.js", get(vendored::htmx))
}

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .route("/system", get(home::system_data))
        .with_state(tx)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(home::main_page))
        .nest("/vendored", vendored_router())
        .nest("/api", api_router())
}
