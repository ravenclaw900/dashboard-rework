mod ext_traits;
mod into_response;
mod websocket;

pub type FullResponse = hyper::Response<http_body_util::Full<hyper::body::Bytes>>;
pub type IncomingReq = hyper::Request<hyper::body::Incoming>;

pub use ext_traits::{RequestExt, ResponseExt, UriExt};
pub use into_response::IntoResponse;
pub use websocket::{upgrade_websocket, WsMessage};

#[macro_export]
macro_rules! router {
    ($req:expr, $state:expr, {
        $( $method:ident $path:literal $(if $cond:expr)? => $handler:expr $(, $opts:ident)*; )*
        _ => $fallback:expr;
    }) => {{
        use $crate::{IntoResponse, UriExt};
        use hyper::Method;

        match ($req.method(), $req.uri().trimmed_path()) {
            $( (&Method::$method, $path) $(if $cond)? => router!(@call $req, $state, $handler $(, $opts)*).into_response(), )*
            _ => $fallback().into_response()
        }
    }};

    (@call $req:expr, $state:expr, $handler:expr) => { $handler() };
    (@call $req:expr, $state:expr, $handler:expr, async) => { $handler().await };
    (@call $req:expr, $state:expr, $handler:expr, async, with_req) => { $handler($req).await };
    (@call $req:expr, $state:expr, $handler:expr, with_req) => { $handler($req) };
    (@call $req:expr, $state:expr, $handler:expr, with_state) => { $handler($state).await };
    (@call $req:expr, $state:expr, $handler:expr, with_req, with_state) => { $handler($req, $state).await };
}
