use once_cell::sync::Lazy;
use serde::Deserialize;
use toml_edit::Document;

mod migrate;

pub static CONFIG: Lazy<Config> = Lazy::new(config);

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

fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("couldn't get path to executable");
    cfgpath.set_file_name("config.toml");

    let toml_str = std::fs::read_to_string(&cfgpath).expect("couldn't read config file");
    let mut toml = toml_str
        .parse::<Document>()
        .expect("config file is invalid");

    loop {
        let migration_done = migrate::migrate(&mut toml);
        if migration_done {
            break;
        }
        std::fs::write(&cfgpath, toml.to_string().as_bytes())
            .expect("failed to migrate config file");
    }

    toml_edit::de::from_document(toml).expect("failed to parse config file")
}
