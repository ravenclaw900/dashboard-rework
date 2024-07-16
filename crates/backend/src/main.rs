use std::net::Ipv6Addr;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::WebSocketStream;

mod handlers;

async fn upgrade_stream(
    stream: TcpStream,
) -> Result<(WebSocketStream<TcpStream>, String), tungstenite::Error> {
    let mut path = None;
    let callback = |req: &Request, resp: Response| {
        path = Some(req.uri().path().to_string());
        Ok(resp)
    };

    let ws = tokio_tungstenite::accept_hdr_async(stream, callback).await?;

    Ok((ws, path.unwrap()))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind((Ipv6Addr::UNSPECIFIED, 5252))
        .await
        .expect("failed to bind to port");

    while let Ok((stream, _)) = listener.accept().await {
        if let Ok((stream, path)) = upgrade_stream(stream).await {}
    }
}
