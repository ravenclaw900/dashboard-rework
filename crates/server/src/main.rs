use flexible_hyper_server_tls::{rustls_helpers, AcceptorBuilder};
use hyper::service::service_fn;
use std::net::{Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;

use config::{CONFIG, VERSION};

mod api;
mod middleware;
mod routers;
mod static_files;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::from_str(&CONFIG.log_level).expect("invalid log level"))
        .init();

    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, CONFIG.port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");

    let builder = AcceptorBuilder::new(listener);

    let mut acceptor = if CONFIG.enable_tls {
        let tls_acceptor =
            rustls_helpers::get_tlsacceptor_from_files(&CONFIG.cert_path, &CONFIG.key_path)
                .expect("failed to read TLS files");

        builder.https(tls_acceptor).build()
    } else {
        builder.build()
    };

    let tx = sysdata::spawn_system_task();

    tracing::info!("Starting dietpi-dashboard v{} on {}", VERSION, addr);

    acceptor
        .serve(service_fn(move |req| routers::router(req, tx.clone())))
        .await;
}
