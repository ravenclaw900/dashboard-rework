use serde::Deserialize;

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
    pub tls: ConfigTls,
    pub auth: ConfigAuth,
}

impl Config {
    pub const DEFAULT: Self = Self {
        port: 5252,
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
