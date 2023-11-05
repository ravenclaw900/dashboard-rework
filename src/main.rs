#![warn(clippy::pedantic, rust_2018_idioms, clippy::nursery)]
// Allow this because axum requires async handlers, even if they don't use async
#![allow(clippy::unused_async)]

use std::net::{Ipv6Addr, SocketAddr};

use color_eyre::Result;

mod frontend;
mod sysdata;
mod types;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Using unspecified IPv6 addr will bind to 0.0.0.0 on both v4 and v6
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 5252));

    let router = frontend::router();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
