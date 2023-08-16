#![warn(clippy::pedantic, rust_2018_idioms, clippy::nursery)]

use std::net::{Ipv6Addr, SocketAddr};

use color_eyre::Result;

mod router;
mod sysdata;
mod types;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 5252));

    let router = router::router();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
