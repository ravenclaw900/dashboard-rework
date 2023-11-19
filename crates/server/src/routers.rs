use axum::{response::Redirect, routing::get, Router};

use super::static_files;

fn static_router() -> Router {
    Router::new()
        .route("/htmx.js", get(static_files::htmx))
        .route("/index.css", get(static_files::index_css))
        .route("/vars.css", get(static_files::css_vars))
}

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .route("/system", get(frontend::system_api))
        .with_state(tx)
}

fn page_router() -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::permanent("/system") }))
        .route("/system", get(frontend::system_page))
}

pub fn router() -> Router {
    Router::new()
        .nest("/static", static_router())
        .nest("/api", api_router())
        .merge(page_router())
}
