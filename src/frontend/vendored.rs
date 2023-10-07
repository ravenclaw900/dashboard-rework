use axum::{http::header, response::IntoResponse};

pub async fn htmx() -> impl IntoResponse {
    let headers = [(header::CONTENT_TYPE, "application/javascript")];
    let body = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vendored/htmx-1.9.6.js"
    ));

    (headers, body)
}
