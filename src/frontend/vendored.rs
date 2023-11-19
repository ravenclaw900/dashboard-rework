use axum::{http::header, response::IntoResponse};

const JS_CONTENT_HEADER: [(header::HeaderName, &str); 1] =
    [(header::CONTENT_TYPE, "application/javascript")];
const CSS_CONTENT_HEADER: [(header::HeaderName, &str); 1] = [(header::CONTENT_TYPE, "text/css")];

macro_rules! vendored_file {
    ($name:ident, $file:literal, $header:ident) => {
        pub async fn $name() -> impl IntoResponse {
            let body = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendored/", $file));

            ($header, body)
        }
    };
}

vendored_file!(open_props, "open-props.css", CSS_CONTENT_HEADER);
vendored_file!(index_css, "index.css", CSS_CONTENT_HEADER);
