use axum::{
    middleware,
    response::Redirect,
    routing::{get, post},
    Router,
};
use config::CONFIG;
use sysdata::RequestTx;

use crate::middleware::login_middleware;

use crate::api;
use crate::static_files;

fn static_router() -> Router {
    Router::new()
        .route("/htmx.js", get(static_files::htmx))
        .route("/xterm.js", get(static_files::xterm))
        .route(
            "/xterm-addon-attach.js",
            get(static_files::xterm_addon_attach),
        )
        .route("/xterm-addon-fit.js", get(static_files::xterm_addon_fit))
        .route("/terminal.js", get(static_files::terminal))
        .route("/index.css", get(static_files::index_css))
        .route("/vars.css", get(static_files::css_vars))
        .route("/xterm.css", get(static_files::xterm_css))
}

fn api_router() -> Router<RequestTx> {
    let mut router = Router::new()
        .route("/process/:pid", post(api::process_signal))
        .route("/terminal", get(api::terminal));

    if CONFIG.auth.enable_auth {
        router = router
            .layer(middleware::from_fn(login_middleware))
            .route("/login", post(api::login));
    }

    router
}

fn frontend_router() -> Router<RequestTx> {
    let mut router = Router::new()
        .route("/", get(|| async { Redirect::permanent("/system") }))
        .route("/process", get(frontend::process::page))
        .route("/process/htmx", get(frontend::process::fragment))
        .route("/system", get(frontend::system::page))
        .route("/system/htmx", get(frontend::system::fragment))
        .route("/management", get(frontend::management::page))
        .route("/terminal", get(frontend::terminal::page));

    if CONFIG.auth.enable_auth {
        router = router
            .layer(middleware::from_fn(login_middleware))
            .route("/login", get(frontend::login::page));
    }

    router
}

pub fn router() -> Router {
    let tx = sysdata::spawn_system_task();

    Router::new()
        .merge(frontend_router())
        .nest("/api", api_router())
        .with_state(tx)
        .nest("/static", static_router())
}
