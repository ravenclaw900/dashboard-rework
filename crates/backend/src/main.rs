use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};

const SOCK_PATH: &str = "/tmp/dpdashboard.sock";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let socket = UnixListener::bind(SOCK_PATH).expect("failed to create socket");

    println!("Starting server");

    tokio::spawn(async move {
        let mut stream = UnixStream::connect(SOCK_PATH).await.unwrap();
        stream.write_all(b"Hello, world!").await.unwrap();
    });

    let (mut conn, addr) = socket.accept().await.unwrap();

    println!("New connection from {addr:?}");

    let mut buf = [0; 24];
    let n = conn.read(&mut buf).await.unwrap();
    dbg!(std::str::from_utf8(&buf[..n]).unwrap());
}
