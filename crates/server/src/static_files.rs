use hyper::header;
use hyper_ext::{FullResponse, IntoResponse, ResponseExt};

const JS_CONTENT_HEADER: &str = "application/javascript";
const CSS_CONTENT_HEADER: &str = "text/css";

macro_rules! static_file {
    ($name:ident, $file:expr, $content_type:expr) => {
        pub fn $name() -> FullResponse {
            let body = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file));

            let mut resp = body.into_response();
            resp.insert_header(header::CONTENT_TYPE, $content_type);
            resp
        }
    };
}

macro_rules! vendored_file {
    ($name:ident, $file:expr, $header:ident) => {
        static_file!($name, concat!("vendored/", $file), $header);
    };
}

vendored_file!(htmx, "htmx-1.9.10.js", JS_CONTENT_HEADER);
vendored_file!(xterm, "xterm-5.3.0.js", JS_CONTENT_HEADER);
vendored_file!(
    xterm_addon_attach,
    "xterm-addon-attach-0.9.0.js",
    JS_CONTENT_HEADER
);
vendored_file!(
    xterm_addon_fit,
    "xterm-addon-fit-0.8.0.js",
    JS_CONTENT_HEADER
);

vendored_file!(css_vars, "vars.css", CSS_CONTENT_HEADER);
vendored_file!(xterm_css, "xterm-5.3.0.css", CSS_CONTENT_HEADER);

static_file!(terminal, "terminal.js", JS_CONTENT_HEADER);
static_file!(index_css, "index.css", CSS_CONTENT_HEADER);
