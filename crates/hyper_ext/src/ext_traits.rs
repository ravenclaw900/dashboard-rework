use std::future::Future;

use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::header::{self, HeaderValue, IntoHeaderName};
use hyper::{StatusCode, Uri};

use crate::{ErrorResponse, HttpResponse, IncomingReq};

pub trait UriExt {
    fn deserialize_query<'de, T: serde::Deserialize<'de>>(&'de self) -> Result<T, ErrorResponse>;

    fn trimmed_path(&self) -> &str;
}

impl UriExt for Uri {
    fn deserialize_query<'de, T: serde::Deserialize<'de>>(&'de self) -> Result<T, ErrorResponse> {
        let query = self.query().unwrap_or_default();
        serde_urlencoded::from_str(query)
            .map_err(|err| ErrorResponse::new_client_err(format!("Bad query: {err}")))
    }

    fn trimmed_path(&self) -> &str {
        if self.path() == "/" {
            return "/";
        }
        self.path().trim_end_matches('/')
    }
}

pub trait ResponseExt {
    fn insert_header_static(&mut self, name: impl IntoHeaderName, val: &'static str);
    fn insert_header(&mut self, name: impl IntoHeaderName, val: &str);
    fn redirect(&mut self, path: &'static str);
}

impl ResponseExt for HttpResponse {
    fn insert_header_static(&mut self, name: impl IntoHeaderName, val: &'static str) {
        let val = HeaderValue::from_static(val);
        self.headers_mut().insert(name, val);
    }

    fn insert_header(&mut self, name: impl IntoHeaderName, val: &str) {
        // Trust that constructed header is valid
        let val = HeaderValue::from_str(val).unwrap();
        self.headers_mut().insert(name, val);
    }

    fn redirect(&mut self, path: &'static str) {
        *self.status_mut() = StatusCode::SEE_OTHER;
        self.insert_header_static(header::LOCATION, path);
    }
}

pub trait RequestExt {
    fn into_body_bytes(self) -> impl Future<Output = Bytes>;
    fn get_cookie(&self, name: &str) -> Option<&str>;
}

impl RequestExt for IncomingReq {
    async fn into_body_bytes(self) -> Bytes {
        let body = self.into_body();
        let collected_body = body.collect().await.unwrap();
        collected_body.to_bytes()
    }

    fn get_cookie(&self, name: &str) -> Option<&str> {
        let cookie_header = self.headers().get(header::COOKIE)?;
        // Cookie header should always be valid UTF-8
        let cookies = cookie_header.to_str().unwrap().split("; ");
        let mut cookie_pairs = cookies.filter_map(|x| x.split_once('='));

        let pair = cookie_pairs.find(|&(k, _)| k == name);
        pair.map(|x| x.1)
    }
}
