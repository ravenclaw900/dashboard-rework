use config::CONFIG;
use hyper::StatusCode;
use hyper_ext::{router, HttpResponse, IncomingReq, IntoResponse, ResponseExt};
use sysdata::RequestTx;

use crate::api;
use crate::middleware::{login_middleware, tracing_middleware};
use crate::static_files;

fn fallback() -> HttpResponse {
    let mut resp = "404 not found".into_response();
    *resp.status_mut() = StatusCode::NOT_FOUND;
    resp
}

fn system_redirect() -> HttpResponse {
    let mut resp = "".into_response();
    resp.redirect("/system");
    resp
}

pub async fn router(
    req: IncomingReq,
    tx: RequestTx,
) -> Result<HttpResponse, std::convert::Infallible> {
    tracing_middleware(&req);

    if CONFIG.enable_auth {
        if let Some(redirect) = login_middleware(&req) {
            return Ok(redirect);
        }
    }

    let resp = router!(req, tx, {
            // Static files
            GET "/static/favicon.png" => static_files::favicon;
            GET "/static/main.css" => static_files::main_css;
            GET "/static/vars.css" => static_files::vars_css;
            GET "/static/xterm.css" => static_files::xterm_css;
            GET "/static/ajaxial.js" => static_files::ajaxial;
            GET "/static/ajaxial-ext.js" => static_files::ajaxial_ext;
            GET "/static/idiomorph.js" => static_files::idiomorph;
            GET "/static/surreal.js" => static_files::surreal;
            GET "/static/xterm.js" => static_files::xterm;
            GET "/static/xterm-addon-attach.js" => static_files::xterm_addon_attach;
            GET "/static/xterm-addon-fit.js" => static_files::xterm_addon_fit;
            // API
            POST "/api/process" => api::process_signal, with_req, with_state;
            GET "/api/terminal" => api::terminal, with_req;
            POST "/api/login" if CONFIG.enable_auth => api::login, async, with_req;
            // Pages
            GET "/" => system_redirect;
            GET "/system" => frontend::system::page, with_state;
            GET "/system/fragment" => frontend::system::fragment, with_state;
            GET "/process" => frontend::process::page, with_req, with_state;
            GET "/process/table" => frontend::process::table, with_req, with_state;
            GET "/process/tbody" => frontend::process::tbody, with_req, with_state;
            GET "/management" => frontend::management::page, with_state;
            GET "/terminal" => frontend::terminal::page;
            GET "/login" if CONFIG.enable_auth => frontend::login::page, with_req;
            _ => fallback;
        }
    );
    Ok(resp)
}
