use flexible_hyper_server_tls::{rustls_helpers, AcceptorBuilder};
use hyper_util::service::TowerToHyperService;
use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;

use config::{CONFIG, VERSION};

mod api;
mod middleware;
mod routers;
mod static_files;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, CONFIG.port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");

    let builder = AcceptorBuilder::new(listener);

    let mut acceptor = if CONFIG.tls.enable_tls {
        let tls_acceptor =
            rustls_helpers::get_tlsacceptor_from_files(&CONFIG.tls.cert_path, &CONFIG.tls.key_path)
                .expect("failed to read TLS files");

        builder.https(tls_acceptor).build()
    } else {
        builder.build()
    };

    let router = routers::router();

    let service = TowerToHyperService::new(router);

    tracing::info!("Starting dietpi-dashboard v{} on {}", VERSION, addr);

    acceptor.serve(service).await;
}
