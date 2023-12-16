use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Redirect},
};
use config::CONFIG;

pub async fn login(body: String) -> impl IntoResponse {
    let Some(pass) = body.strip_prefix("pass=") else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    if !auth::test_password(pass) {
        return Redirect::to("/login?incorrect=").into_response();
    }

    let token = auth::create_token();

    let cookie_header = format!(
        "token={}; SameSite=Lax; Max-Age={}",
        token, CONFIG.auth.expiry
    );

    ([(header::SET_COOKIE, cookie_header)], Redirect::to("/")).into_response()
}
