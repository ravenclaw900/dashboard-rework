use std::future::Future;

use crate::{FullResponse, IncomingReq, IntoResponse};

pub fn upgrade_websocket<F, Fut>(mut req: IncomingReq, callback: F) -> FullResponse
where
    F: FnOnce(hyper_tungstenite::HyperWebsocketStream) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    if !hyper_tungstenite::is_upgrade_request(&req) {
        let mut resp = "Expected websocket upgrade".into_response();
        *resp.status_mut() = hyper::StatusCode::BAD_REQUEST;
        return resp;
    }

    let Ok((resp, websocket)) = hyper_tungstenite::upgrade(&mut req, None) else {
        let mut resp = "Bad websocket upgrade".into_response();
        *resp.status_mut() = hyper::StatusCode::BAD_REQUEST;
        return resp;
    };

    tokio::spawn(async {
        if let Ok(stream) = websocket.await {
            callback(stream).await;
        }
    });

    resp
}

pub use hyper_tungstenite::tungstenite::Message as WsMessage;
