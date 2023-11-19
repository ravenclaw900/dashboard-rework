use super::{pages, vendored};
use crate::sysdata;

use axum::routing::get;
use axum::Router;

fn vendored_router() -> Router {
    Router::new()
        .route("/htmx.js", get(vendored::htmx))
        .route("/index.css", get(vendored::index_css))
        .route("/vars.css", get(vendored::css_vars))
}

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .route("/system", get(pages::home::system_api))
        .with_state(tx)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(pages::home::system_page))
        .nest("/api", api_router())
        .nest("/vendored", vendored_router())
}
