#[cfg(feature = "frontend")]
pub type Config = frontend::v1::FrontendConfig;
#[cfg(feature = "frontend")]
pub const CONFIG_NAME: &str = "config-frontend.toml";

#[cfg(feature = "frontend")]
pub mod frontend;
pub mod migrate;
mod v0;
