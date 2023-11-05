use tokio::sync::mpsc::Sender;

use super::{pages, vendored};
use crate::sysdata;

use axum::routing::get;
use axum::Router;

fn vendored_router() -> Router {
    Router::new()
        .route("/htmx.js", get(vendored::htmx))
        .route("/index.css", get(vendored::index_css))
        .route("/open-props.css", get(vendored::open_props))
}

fn api_router() -> Router<Sender<sysdata::Request>> {
    Router::new().route("/system", get(pages::home::system_api))
}

pub fn router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .route("/", get(pages::home::system_page))
        .nest("/api", api_router())
        .with_state(tx)
        .nest("/vendored", vendored_router())
}
