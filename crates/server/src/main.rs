use flexible_hyper_server_tls::{tlsconfig, HyperHttpOrHttpsAcceptor};
use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;

use config::CONFIG;

mod api;
mod middleware;
mod routers;
mod static_files;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, CONFIG.port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");

    let acceptor = if CONFIG.tls.enable_tls {
        let tls_acceptor = tlsconfig::get_tlsacceptor_from_files(
            &CONFIG.tls.cert_path,
            &CONFIG.tls.key_path,
            tlsconfig::HttpProtocol::Http1,
        )
        .expect("failed to read TLS files");

        HyperHttpOrHttpsAcceptor::new_https(
            listener,
            tls_acceptor,
            std::time::Duration::from_secs(10),
        )
    } else {
        HyperHttpOrHttpsAcceptor::new_http(listener)
    };

    let router = routers::router();

    axum::Server::builder(acceptor)
        .serve(router.into_make_service())
        .await
        .expect("failed to start server");
}
