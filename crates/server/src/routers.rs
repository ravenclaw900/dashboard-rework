use axum::{
    middleware,
    response::Redirect,
    routing::{get, post},
    Router,
};

use crate::middleware::login_middleware;

use super::api;
use super::static_files;

fn static_router() -> Router {
    Router::new()
        .route("/htmx.js", get(static_files::htmx))
        .route("/iconify.js", get(static_files::iconify))
        .route("/index.css", get(static_files::index_css))
        .route("/vars.css", get(static_files::css_vars))
}

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .route("/system", get(frontend::system_api))
        .route("/process", get(frontend::process_api))
        .route("/process/:pid", post(api::process_signal))
        .layer(middleware::from_fn(login_middleware))
        .route("/login", post(api::login))
        .with_state(tx)
}

fn page_router() -> Router {
    Router::new()
        .route("/", get(|| async { Redirect::permanent("/system") }))
        .route("/system", get(frontend::system_page))
        .route("/process", get(frontend::process_page))
        .layer(middleware::from_fn(login_middleware))
        .route("/login", get(frontend::login_page))
}

pub fn router() -> Router {
    Router::new()
        .nest("/static", static_router())
        .nest("/api", api_router())
        .merge(page_router())
}
