use std::future::Future;

use crate::{ErrorResponse, FullResponse, IncomingReq, IntoResponse};

pub fn upgrade_websocket<F, Fut>(mut req: IncomingReq, callback: F) -> FullResponse
where
    F: FnOnce(hyper_tungstenite::HyperWebsocketStream) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    if !hyper_tungstenite::is_upgrade_request(&req) {
        return ErrorResponse::new_client_err("Expected websocket upgrade").into_response();
    }

    let Ok((resp, websocket)) = hyper_tungstenite::upgrade(&mut req, None) else {
        return ErrorResponse::new_client_err("Bad websocket upgrade headers").into_response();
    };

    tokio::spawn(async {
        if let Ok(stream) = websocket.await {
            callback(stream).await;
        }
    });

    resp
}

pub use hyper_tungstenite::tungstenite::Message as WsMessage;
