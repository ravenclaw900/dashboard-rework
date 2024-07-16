use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub async fn handle_connection(ws: WebSocketStream<TcpStream>, path: &str) {
    match path {
        "/" => handle_main(ws).await,
        _ => {}
    }
}

async fn handle_main(mut ws: WebSocketStream<TcpStream>) {
    while let Some(msg) = ws.next().await {}
}
