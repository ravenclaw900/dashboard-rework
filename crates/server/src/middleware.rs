use hyper_ext::{HttpResponse, IncomingReq, IntoResponse, RequestExt, ResponseExt};

fn validate_token_cookie(req: &IncomingReq) -> bool {
    let Some(token_cookie) = req.get_cookie("token") else {
        return false;
    };

    auth::verify_token(token_cookie)
}

pub fn login_middleware(req: &IncomingReq) -> Option<HttpResponse> {
    if validate_token_cookie(req) {
        // Login is good, no need to redirect
        None
    } else if req.headers().contains_key("Ajxl-Request") {
        // Insert a script to do redirect if request is from ajaxial
        let resp = "<script>window.location.href='/login'</script>".into_response();
        Some(resp)
    } else {
        // Otherwise just do a normal redirect
        let mut resp = "Redirecting".into_response();
        resp.redirect("/login");
        Some(resp)
    }
}

pub fn tracing_middleware(req: &IncomingReq) {
    tracing::debug!("Request to {}", req.uri().path());
}
