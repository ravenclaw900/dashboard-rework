use std::fmt::Display;

use hyper::StatusCode;

use crate::IntoResponse;

#[derive(Debug)]
pub struct ErrorResponse {
    msg: String,
    code: StatusCode,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.code.canonical_reason().unwrap_or("Error"),
            self.msg
        )
    }
}

impl ErrorResponse {
    pub const CHANNEL_MSG: &'static str = "Failed to request system data";
    pub const QUERY_MSG: &'static str = "Bad query";

    pub fn new_client_err(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            code: StatusCode::BAD_REQUEST,
        }
    }

    pub fn new_server_err(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn new_custom(msg: &str, code: StatusCode) -> Self {
        Self {
            msg: msg.to_string(),
            code,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> crate::HttpResponse {
        let mut resp = self.msg.into_response();
        *resp.status_mut() = self.code;
        resp
    }
}
