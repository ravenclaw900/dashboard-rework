use std::future::Future;

use hyper_tungstenite::HyperWebsocketStream;

use crate::{ErrorResponse, HttpResponse, IncomingReq, IntoResponse};

pub fn upgrade_websocket<F, Fut>(
    req: IncomingReq,
    callback: F,
) -> Result<HttpResponse, ErrorResponse>
where
    F: FnOnce(HyperWebsocketStream) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Check to see if request is actually a websocket upgrade
    if !hyper_tungstenite::is_upgrade_request(&req) {
        return Err(ErrorResponse::new_client_err("Expected websocket upgrade"));
    }

    let Ok((resp, websocket)) = hyper_tungstenite::upgrade(req, None) else {
        return Err(ErrorResponse::new_client_err("Bad websocket upgrade"));
    };

    tokio::spawn(async {
        if let Ok(ws) = websocket.await {
            callback(ws).await;
        }
    });

    Ok(resp.into_response())
}

pub use hyper_tungstenite::tungstenite::Message as WsMessage;
