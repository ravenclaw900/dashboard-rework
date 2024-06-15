use serde::Deserialize;
use tracing::level_filters::LevelFilter;

#[derive(Deserialize)]
pub struct ConfigTls {
    pub enable_tls: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Deserialize)]
pub struct ConfigAuth {
    pub enable_auth: bool,
    pub privkey_path: String,
    pub pubkey_path: String,
    pub hash: String,
    pub expiry: u64,
}

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    #[serde(deserialize_with = "deser_levelfilter")]
    pub log_level: LevelFilter,
    pub tls: ConfigTls,
    pub auth: ConfigAuth,
}

impl Config {
    pub const DEFAULT: Self = Self {
        port: 5252,
        log_level: LevelFilter::INFO,
        tls: ConfigTls {
            enable_tls: false,
            cert_path: String::new(),
            key_path: String::new(),
        },
        auth: ConfigAuth {
            enable_auth: false,
            privkey_path: String::new(),
            pubkey_path: String::new(),
            hash: String::new(),
            expiry: 3600,
        },
    };
}

fn deser_levelfilter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Unexpected};
    use std::str::FromStr;

    let filter_str = String::deserialize(deserializer)?;
    LevelFilter::from_str(&filter_str).map_err(|_| {
        de::Error::invalid_value(
            Unexpected::Str(&filter_str),
            &"off, error, warn, info, debug",
        )
    })
}
