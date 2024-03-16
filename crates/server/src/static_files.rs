use axum::{http::header, response::IntoResponse};

const JS_CONTENT_HEADER: [(header::HeaderName, &str); 1] =
    [(header::CONTENT_TYPE, "application/javascript")];
const CSS_CONTENT_HEADER: [(header::HeaderName, &str); 1] = [(header::CONTENT_TYPE, "text/css")];

macro_rules! vendored_file {
    ($name:ident, $file:literal, $header:ident) => {
        pub async fn $name() -> impl IntoResponse {
            let body = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file));

            ($header, body)
        }
    };
}

vendored_file!(htmx, "htmx-1.9.10.js", JS_CONTENT_HEADER);
vendored_file!(xterm, "xterm-5.3.0.js", JS_CONTENT_HEADER);
vendored_file!(
    xterm_addon_attach,
    "xterm-addon-attach-0.9.0.js",
    JS_CONTENT_HEADER
);

vendored_file!(terminal, "terminal.js", JS_CONTENT_HEADER);

vendored_file!(css_vars, "vars.css", CSS_CONTENT_HEADER);
vendored_file!(xterm_css, "xterm-5.3.0.css", CSS_CONTENT_HEADER);
vendored_file!(index_css, "index.css", CSS_CONTENT_HEADER);
