use async_trait::async_trait;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::header::{self, AsHeaderName, HeaderValue, IntoHeaderName};
use hyper::{StatusCode, Uri};

use crate::{HttpResponse, IncomingReq};

pub trait UriExt {
    fn deserialize_query<'de, T: serde::Deserialize<'de>>(
        &'de self,
    ) -> Result<T, serde_urlencoded::de::Error>;

    fn trimmed_path(&self) -> &str;
}

impl UriExt for Uri {
    fn deserialize_query<'de, T: serde::Deserialize<'de>>(
        &'de self,
    ) -> Result<T, serde_urlencoded::de::Error> {
        let query = self.query().unwrap_or_default();
        serde_urlencoded::from_str(query)
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

#[async_trait]
pub trait RequestExt {
    async fn into_body_bytes(self) -> Bytes;
    fn check_header(&self, name: impl AsHeaderName, f: impl FnOnce(&HeaderValue) -> bool) -> bool;
}

#[async_trait]
impl RequestExt for IncomingReq {
    async fn into_body_bytes(self) -> Bytes {
        let body = self.into_body();
        let collected_body = body.collect().await.unwrap();
        collected_body.to_bytes()
    }

    fn check_header(&self, name: impl AsHeaderName, f: impl FnOnce(&HeaderValue) -> bool) -> bool {
        self.headers().get(name).is_some_and(f)
    }
}
