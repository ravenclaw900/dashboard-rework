use super::migrate::Migrate;
use serde::Deserialize;
use std::path::PathBuf;

pub mod v1 {
    use super::*;
    use crate::versions::v0::ConfigV0;

    #[derive(Deserialize)]
    pub struct FrontendConfig {
        pub port: u16,
        pub log_level: String,
        // TLS options
        pub enable_tls: bool,
        pub cert_path: PathBuf,
        pub key_path: PathBuf,
        // Auth options
        pub enable_auth: bool,
        pub privkey_path: PathBuf,
        pub pubkey_path: PathBuf,
        pub hash: String,
        pub expiry: u32,
    }

    impl Default for FrontendConfig {
        fn default() -> Self {
            Self {
                port: 5252,
                log_level: "info".to_string(),
                enable_tls: false,
                cert_path: PathBuf::new(),
                key_path: PathBuf::new(),
                enable_auth: false,
                privkey_path: PathBuf::new(),
                pubkey_path: PathBuf::new(),
                hash: String::new(),
                expiry: 3600,
            }
        }
    }

    impl From<ConfigV0> for FrontendConfig {
        fn from(value: ConfigV0) -> Self {
            let default = Self::default();

            Self {
                port: value.port.unwrap_or(default.port),
                log_level: value.log_level.unwrap_or(default.log_level),
                enable_tls: value.tls.unwrap_or(default.enable_tls),
                cert_path: value.cert.unwrap_or(default.cert_path),
                key_path: value.key.unwrap_or(default.key_path),
                enable_auth: value.pass.unwrap_or(default.enable_auth),
                privkey_path: default.privkey_path,
                pubkey_path: default.pubkey_path,
                hash: value.hash.unwrap_or(default.hash),
                expiry: value.expiry.unwrap_or(default.expiry),
            }
        }
    }

    impl Migrate for FrontendConfig {
        type From = ConfigV0;

        const VERSION: i64 = 1;
    }
}
