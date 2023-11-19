use std::net::{Ipv6Addr, SocketAddr};

mod routers;
mod static_files;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 5252));

    let router = routers::router();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("failed to start server");
}
