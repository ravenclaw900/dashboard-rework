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
    pub fn new_client_err(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            code: StatusCode::BAD_REQUEST,
        }
    }

    pub fn new_server_err(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn new_custom(msg: impl Into<String>, code: StatusCode) -> Self {
        Self {
            msg: msg.into(),
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
