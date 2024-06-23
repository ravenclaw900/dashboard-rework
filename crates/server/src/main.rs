use flexible_hyper_server_tls::{rustls_helpers, AcceptorBuilder};
use hyper::service::service_fn;
use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;

use config::{CONFIG, VERSION};

mod api;
mod middleware;
mod routers;
mod static_files;

// The point of this function is to have logging available while reading the config file,
// which requires a subscriber to be set up before reading the config
fn init_logging() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{fmt, reload};

    let (filter, reload_handle) = reload::Layer::new(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let log_level = CONFIG.log_level;

    _ = reload_handle.modify(|filter| *filter = log_level);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_logging();

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

    let tx = sysdata::spawn_system_task();

    tracing::info!("Starting dietpi-dashboard v{} on {}", VERSION, addr);

    acceptor
        .serve(service_fn(move |req| routers::router(req, tx.clone())))
        .await;
}
