use hyper::HeaderMap;
use hyper_ext::{FullResponse, IncomingReq, IntoResponse, ResponseExt};

fn validate_token_cookie(headers: &HeaderMap) -> bool {
    // Get Cookie header and attempt to convert it to a string, returning false if either step fails
    let Some(cookie_header) = headers.get("Cookie").and_then(|x| x.to_str().ok()) else {
        return false;
    };

    // Split by '; ' to get cookie pairs, then split each of those by '=' to get key and value
    let mut cookies = cookie_header.split("; ").filter_map(|x| x.split_once('='));

    let Some(token_cookie) = cookies.find(|x| x.0 == "token") else {
        return false;
    };

    auth::verify_token(token_cookie.1)
}

// pub async fn login_middleware(req: Request<Body>, next: Next) -> Response {
//     if validate_token_cookie(req.headers()) {
//         // Generate response from other middleware/handler
//         next.run(req).await
//     } else if req.headers().contains_key("HX-Request") {
//         // Use htmx to do client-side redirect if request is from htmx
//         [("HX-Redirect", "/login")].into_response()
//     } else {
//         // Otherwise just do a normal redirect
//         Redirect::to("/login").into_response()
//     }
// }

pub fn login_middleware_2(req: &IncomingReq) -> Option<FullResponse> {
    if validate_token_cookie(req.headers()) {
        // Login is good, no need to redirect
        None
    } else if req.headers().contains_key("HX-Request") {
        // Use htmx to do client-side redirect if request is from htmx
        let mut resp = "".into_response();
        resp.insert_header("HX-Redirect", "/login");
        Some(resp)
    } else {
        // Otherwise just do a normal redirect
        let mut resp = "".into_response();
        resp.redirect("/login");
        Some(resp)
    }
}

// pub async fn tracing_middleware(req: Request<Body>, next: Next) -> Response {
//     use tracing::Instrument;

//     // TODO: add remote addr here
//     let span = tracing::info_span!("request");
//     async {
//         tracing::debug!("Request to {}", req.uri().path());
//         next.run(req).await
//     }
//     .instrument(span)
//     .await
// }
