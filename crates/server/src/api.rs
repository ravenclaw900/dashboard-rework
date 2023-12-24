use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use config::CONFIG;
use serde::Deserialize;
use sysdata::{Request, RequestTx};

pub async fn login(body: String) -> Response {
    let Some(pass) = body.strip_prefix("pass=") else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    if !auth::test_password(pass) {
        return Redirect::to("/login?incorrect=").into_response();
    }

    let token = auth::create_token();

    let cookie_header = format!(
        "token={}; SameSite=Lax; Max-Age={}; HttpOnly; Path=/",
        token, CONFIG.auth.expiry
    );

    ([(header::SET_COOKIE, cookie_header)], Redirect::to("/")).into_response()
}

#[derive(Deserialize)]
pub struct ProcessSignalQuery {
    signal: sysdata::types::ProcessSignal,
}

pub async fn process_signal(
    State(tx): State<RequestTx>,
    Path(pid): Path<usize>,
    signal: Query<ProcessSignalQuery>,
) {
    tx.send(Request::ProcessSignal(pid, signal.0.signal))
        .await
        .unwrap();
}
