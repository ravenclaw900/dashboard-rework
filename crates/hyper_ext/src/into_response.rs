use crate::{FullResponse, ResponseExt};
use http_body_util::Full;
use hyper::header;
use hyper::Response;

pub trait IntoResponse {
    fn into_response(self) -> FullResponse;
}

macro_rules! impl_for_into_full {
    ($typ:ty) => {
        impl IntoResponse for $typ {
            fn into_response(self) -> FullResponse {
                Response::new(Full::from(self))
            }
        }
    };
}

impl_for_into_full!(&'static [u8]);
impl_for_into_full!(&'static str);
impl_for_into_full!(String);

impl IntoResponse for FullResponse {
    fn into_response(self) -> FullResponse {
        self
    }
}

impl IntoResponse for maud::Markup {
    fn into_response(self) -> FullResponse {
        let mut resp = self.0.into_response();
        resp.insert_header_static(header::CONTENT_TYPE, "text/html");
        resp
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> FullResponse {
        match self {
            Ok(t) => t.into_response(),
            Err(e) => e.into_response(),
        }
    }
}

impl IntoResponse for () {
    fn into_response(self) -> FullResponse {
        [].into_response()
    }
}
