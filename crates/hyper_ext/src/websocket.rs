use std::future::Future;

use hyper::{header, StatusCode};
use hyper_util::rt::TokioIo;
use tokio_tungstenite::tungstenite::handshake::derive_accept_key;
use tokio_tungstenite::tungstenite::protocol::Role;
use tokio_tungstenite::WebSocketStream;

use crate::{ErrorResponse, HttpResponse, IncomingReq, IntoResponse, RequestExt, ResponseExt};

type WebSocket = WebSocketStream<TokioIo<hyper::upgrade::Upgraded>>;

// The main reason I'm doing this myself and not using a library like `hyper-tungstenite`
// is because I need to return a ResponseBody, while `hyper-tungstenite` returns a http_body_util::Full
// and there seems to be no way to easily convert between them
pub fn upgrade_websocket<F, Fut>(
    req: IncomingReq,
    callback: F,
) -> Result<HttpResponse, ErrorResponse>
where
    F: FnOnce(WebSocket) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Check to see if request is actually a websocket upgrade
    if !(req.check_header(header::CONNECTION, |val| val.as_bytes() == b"Upgrade")
        && req.check_header(header::UPGRADE, |val| val.as_bytes() == b"websocket")
        && req.check_header("Sec-WebSocket-Version", |val| val.as_bytes() == b"13"))
    {
        return Err(ErrorResponse::new_client_err(
            "Expected websocket v13 upgrade",
        ));
    }

    let Some(key) = req.headers().get("Sec-WebSocket-Key") else {
        return Err(ErrorResponse::new_client_err("Missing websocket key"));
    };

    let mut resp = "Switching to websocket".into_response();
    *resp.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
    resp.insert_header_static(header::CONNECTION, "Upgrade");
    resp.insert_header_static(header::UPGRADE, "websocket");
    resp.insert_header("Sec-WebSocket-Accept", &derive_accept_key(key.as_bytes()));

    tokio::spawn(async {
        if let Ok(upgraded) = hyper::upgrade::on(req).await {
            let stream =
                WebSocketStream::from_raw_socket(TokioIo::new(upgraded), Role::Server, None);

            callback(stream.await).await;
        }
    });

    Ok(resp)
}

pub use tokio_tungstenite::tungstenite::Message as WsMessage;
