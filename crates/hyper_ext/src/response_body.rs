use crate::{HttpResponse, ResponseExt};
use futures_core::FusedStream;
use hyper::body::{Body, Bytes, Frame, SizeHint};
use hyper::header;
use hyper::Response;
use std::pin::Pin;
use std::task::Poll;

pub trait IntoResponse {
    fn into_response(self) -> HttpResponse;
}

macro_rules! impl_for_into_bytes {
    ($typ:ty) => {
        impl IntoResponse for $typ {
            fn into_response(self) -> HttpResponse {
                if self.is_empty() {
                    Response::new(ResponseBody::Empty)
                } else {
                    Response::new(ResponseBody::Full(self.into()))
                }
            }
        }
    };
}

impl_for_into_bytes!(&'static [u8]);
impl_for_into_bytes!(&'static str);
impl_for_into_bytes!(String);

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
        Response::new(ResponseBody::Empty)
    }
}

pub enum ResponseBody {
    Empty,
    Full(Bytes),
    // Yes, I'm caving and using a Pin<Box>
    // We need the type erasure, and not having to deal with pin projection is a bonus
    Streaming(Pin<Box<dyn FusedStream<Item = Bytes> + Send>>),
}

impl Body for ResponseBody {
    type Data = Bytes;

    type Error = std::convert::Infallible;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.get_mut();

        match this {
            Self::Empty => Poll::Ready(None),
            Self::Full(data) => {
                // Take the data out of self, then set it to Empty so it returns None on future polls
                let data = std::mem::take(data);
                *this = Self::Empty;

                Poll::Ready(Some(Ok(Frame::data(data))))
            }
            Self::Streaming(stream) => match stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(data)) => Poll::Ready(Some(Ok(Frame::data(data)))),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            },
        }
    }

    fn is_end_stream(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Full(_) => false,
            Self::Streaming(stream) => stream.as_ref().is_terminated(),
        }
    }

    fn size_hint(&self) -> SizeHint {
        match self {
            Self::Empty => SizeHint::with_exact(0),
            Self::Full(data) => SizeHint::with_exact(data.len() as u64),
            Self::Streaming(stream) => {
                if stream.as_ref().is_terminated() {
                    SizeHint::with_exact(0)
                } else {
                    SizeHint::default()
                }
            }
        }
    }
}
