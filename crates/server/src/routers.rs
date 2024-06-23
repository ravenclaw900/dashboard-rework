use config::CONFIG;
use hyper::StatusCode;
use hyper_ext::{router, FullResponse, IncomingReq, IntoResponse, ResponseExt};
use sysdata::RequestTx;

use crate::middleware::login_middleware;

use crate::api;
use crate::middleware::tracing_middleware;
use crate::static_files;

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
    tracing_middleware(&req);

    if CONFIG.auth.enable_auth {
        if let Some(redirect) = login_middleware(&req) {
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
