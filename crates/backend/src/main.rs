use std::net::Ipv6Addr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UnixListener, UnixStream};
use tokio_tungstenite::tungstenite::handshake::server::Request;

const SOCK_PATH: &str = "/tmp/dpdashboard.sock";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind((Ipv6Addr::UNSPECIFIED, 5252))
        .await
        .expect("failed to bind to port");

    while let Ok((stream, _)) = listener.accept().await {
        let mut uri = None;
        if let Ok(stream) = tokio_tungstenite::accept_hdr_async(stream, |req: &Request, resp| {
            uri = Some(req.uri().clone());
            Ok(resp)
        })
        .await
        {
            let uri = uri.unwrap();
        }
    }
}
