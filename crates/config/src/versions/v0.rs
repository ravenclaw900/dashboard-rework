use serde::Deserialize;
use std::path::PathBuf;

use super::migrate::Migrate;

#[derive(Deserialize)]
pub struct ConfigV0 {
    pub port: Option<u16>,
    pub log_level: Option<String>,
    pub tls: Option<bool>,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
    pub pass: Option<bool>,
    pub hash: Option<String>,
    pub secret: Option<String>,
    pub expiry: Option<u32>,
}

impl Migrate for ConfigV0 {
    type From = Self;

    const VERSION: i64 = 0;
}
