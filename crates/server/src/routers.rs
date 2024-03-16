use axum::{
    middleware,
    response::Redirect,
    routing::{get, post},
    Router,
};
use config::CONFIG;

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

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();

    let mut router = Router::new()
        .route("/system", get(frontend::system_api))
        .route("/process", get(frontend::process_api))
        .route("/process/:pid", post(api::process_signal))
        .route("/terminal", get(api::terminal))
        .with_state(tx);

    if CONFIG.auth.enable_auth {
        router = router
            .layer(middleware::from_fn(login_middleware))
            .route("/login", post(api::login));
    }

    router
}

fn page_router() -> Router {
    let mut router = Router::new()
        .route("/", get(|| async { Redirect::permanent("/system") }))
        .route("/system", get(frontend::system_page))
        .route("/process", get(frontend::process_page))
        .route("/terminal", get(frontend::terminal_page));

    if CONFIG.auth.enable_auth {
        router = router
            .layer(middleware::from_fn(login_middleware))
            .route("/login", get(frontend::login_page));
    }

    router
}

pub fn router() -> Router {
    Router::new()
        .nest("/static", static_router())
        .nest("/api", api_router())
        .merge(page_router())
}
