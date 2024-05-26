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
