use crate::{HttpResponse, ResponseExt};
use futures_util::stream::{Stream, StreamExt};
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::body::{Bytes, Frame};
use hyper::header;
use hyper::Response;

pub trait IntoResponse {
    fn into_response(self) -> HttpResponse;
}

macro_rules! impl_for_into_full {
    ($typ:ty) => {
        impl IntoResponse for $typ {
            fn into_response(self) -> HttpResponse {
                Response::new(Full::from(self).boxed_unsync())
            }
        }
    };
}

impl_for_into_full!(&'static [u8]);
impl_for_into_full!(&'static str);
impl_for_into_full!(String);

impl IntoResponse for Response<Full<Bytes>> {
    fn into_response(self) -> HttpResponse {
        let (parts, body) = self.into_parts();
        Response::from_parts(parts, body.boxed_unsync())
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> HttpResponse {
        self
    }
}

impl IntoResponse for maud::Markup {
    fn into_response(self) -> HttpResponse {
        let mut resp = self.0.into_response();
        resp.insert_header_static(header::CONTENT_TYPE, "text/html");
        resp
    }
}

impl IntoResponse for datastar::response::FullDatastarResponse {
    fn into_response(self) -> HttpResponse {
        let mut resp = self.into_string().into_response();

        resp.insert_header_static(header::CONNECTION, "keep-alive");
        resp.insert_header_static(header::CONTENT_TYPE, "text/event-stream");
        resp.insert_header_static(header::CACHE_CONTROL, "no-cache");

        resp
    }
}

impl<S> IntoResponse for datastar::response::StreamingDatastarResponse<S>
where
    S: Stream<Item = datastar::message::DatastarMessage> + Send + 'static,
{
    fn into_response(self) -> HttpResponse {
        let stream = self
            .into_stream()
            .map(|x| Ok(Frame::data(Bytes::from(x.into_string()))));
        let body = StreamBody::new(stream).boxed_unsync();

        let mut resp = Response::new(body);

        resp.insert_header_static(header::CONNECTION, "keep-alive");
        resp.insert_header_static(header::CONTENT_TYPE, "text/event-stream");
        resp.insert_header_static(header::CACHE_CONTROL, "no-cache");

        resp
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> HttpResponse {
        match self {
            Ok(t) => t.into_response(),
            Err(e) => e.into_response(),
        }
    }
}

impl IntoResponse for () {
    fn into_response(self) -> HttpResponse {
        [].into_response()
    }
}
