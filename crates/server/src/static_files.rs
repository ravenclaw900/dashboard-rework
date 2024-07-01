use hyper::header;
use hyper_ext::{HttpResponse, IntoResponse, ResponseExt};

const JS_CONTENT_HEADER: &str = "application/javascript";
const CSS_CONTENT_HEADER: &str = "text/css";

macro_rules! static_file {
    ($name:ident, $file:expr, $content_type:expr) => {
        pub fn $name() -> HttpResponse {
            let body = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file));

            let mut resp = body.into_response();
            resp.insert_header_static(header::CONTENT_TYPE, $content_type);
            resp
        }
    };
}

// Vendored libraries
// HTMX
static_file!(htmx, "vendored/htmx-1.9.10.js", JS_CONTENT_HEADER);
// xterm
static_file!(xterm, "vendored/xterm-5.3.0.js", JS_CONTENT_HEADER);
static_file!(xterm_css, "vendored/xterm-5.3.0.css", CSS_CONTENT_HEADER);
static_file!(
    xterm_addon_attach,
    "vendored/xterm-addon-attach-0.9.0.js",
    JS_CONTENT_HEADER
);
static_file!(
    xterm_addon_fit,
    "vendored/xterm-addon-fit-0.8.0.js",
    JS_CONTENT_HEADER
);

// Other static files
static_file!(css_vars, "vars.css", CSS_CONTENT_HEADER);
static_file!(index_css, "main.css", CSS_CONTENT_HEADER);
static_file!(favicon, "favicon.png", "image/png");
