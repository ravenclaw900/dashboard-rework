use config::CONFIG;
use hyper::StatusCode;
use hyper_ext::{router, FullResponse, IncomingReq, IntoResponse, ResponseExt};
use sysdata::RequestTx;

use crate::middleware::login_middleware_2;

use crate::api;
use crate::static_files;

// fn static_router() -> Router {
//     Router::new()
//         .route("/htmx.js", get(static_files::htmx))
//         .route("/xterm.js", get(static_files::xterm))
//         .route(
//             "/xterm-addon-attach.js",
//             get(static_files::xterm_addon_attach),
//         )
//         .route("/xterm-addon-fit.js", get(static_files::xterm_addon_fit))
//         .route("/terminal.js", get(static_files::terminal))
//         .route("/index.css", get(static_files::index_css))
//         .route("/vars.css", get(static_files::css_vars))
//         .route("/xterm.css", get(static_files::xterm_css))
// }
//
// fn api_router() -> Router<RequestTx> {
//     let mut router = Router::new()
//         .route("/process/:pid", post(api::process_signal))
//         .route("/terminal", get(api::terminal));

//     if CONFIG.auth.enable_auth {
//         router = router
//             .layer(middleware::from_fn(login_middleware))
//             .route("/login", post(api::login));
//     }

//     router
// }

// fn frontend_router() -> Router<RequestTx> {
//     let mut router = Router::new()
//         .route("/", get(|| async { Redirect::permanent("/system") }))
//         .route("/process", get(frontend::process::page))
//         .route("/process/htmx", get(frontend::process::fragment))
//         .route("/system", get(frontend::system::page))
//         .route("/system/htmx", get(frontend::system::fragment))
//         // .route("/management", get(frontend::management::page))
//         .route("/terminal", get(frontend::terminal::page));

//     if CONFIG.auth.enable_auth {
//         router = router.layer(middleware::from_fn(login_middleware))
//         // .route("/login", get(frontend::login::page));
//     }

//     router
// }

// pub fn router() -> Router {
//     let tx = sysdata::spawn_system_task();

//     Router::new()
//         .merge(frontend_router())
//         .nest("/api", api_router())
//         .with_state(tx)
//         // .nest("/static", static_router())
//         .layer(middleware::from_fn(tracing_middleware))
// }

fn fallback() -> FullResponse {
    let mut resp = "404 not found".into_response();
    *resp.status_mut() = StatusCode::NOT_FOUND;
    resp
}

fn system_redirect() -> FullResponse {
    let mut resp = "".into_response();
    resp.redirect("/system");
    resp
}

pub async fn router(
    req: IncomingReq,
    tx: RequestTx,
) -> Result<FullResponse, std::convert::Infallible> {
    if CONFIG.auth.enable_auth {
        if let Some(redirect) = login_middleware_2(&req) {
            return Ok(redirect);
        }
    }

    let resp = router!(req, tx, {
            // Static files
            GET "/static/index.css" => static_files::index_css;
            GET "/static/vars.css" => static_files::css_vars;
            GET "/static/xterm.css" => static_files::xterm_css;
            GET "/static/htmx.js" => static_files::htmx;
            GET "/static/xterm.js" => static_files::xterm;
            GET "/static/xterm-addon-attach.js" => static_files::xterm_addon_attach;
            GET "/static/xterm-addon-fit.js" => static_files::xterm_addon_fit;
            GET "/static/terminal.js" => static_files::terminal;
            // API
            POST "/api/process" => api::process_signal, with_req, with_state;
            GET "/api/terminal" => api::terminal, with_req;
            POST "/api/login" if CONFIG.auth.enable_auth => api::login, async, with_req;
            // Pages
            GET "/" => system_redirect;
            GET "/system" => frontend::system::page, with_state;
            GET "/system/htmx" => frontend::system::fragment, with_state;
            GET "/process" => frontend::process::page, with_state;
            GET "/process/htmx" => frontend::process::fragment, with_req, with_state;
            GET "/management" => frontend::management::page, with_state;
            GET "/terminal" => frontend::terminal::page;
            GET "/login" if CONFIG.auth.enable_auth => frontend::login::page, with_req;
            _ => fallback;
        }
    );
    Ok(resp)
}
