use once_cell::sync::Lazy;
use std::path::Path;
use toml_edit::DocumentMut;
use versions::{migrate::migrate_config, Config};

mod versions;

pub static CONFIG: Lazy<Config> = Lazy::new(config);
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn write_config_file(path: &Path, config: &Config) {
    let config_file = generate_config_file(config);
    std::fs::write(path, config_file.as_bytes()).expect("failed to write new config file");
}

macro_rules! generate_config_file {
    ($template:literal, $($key:ident = $val:expr),*) => {{
        use serde::Serialize;
        use toml_edit::ser::ValueSerializer;

        $( let $key = Serialize::serialize(&($val), ValueSerializer::new()).unwrap(); )*

        format!(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", $template)), $($key = $key),*)
    }};
}

#[cfg(feature = "frontend")]
pub fn generate_config_file(config: &Config) -> String {
    generate_config_file!(
        "config-frontend.template.toml",
        port = config.port,
        log_level = config.log_level,
        enable_tls = config.enable_tls,
        key_path = config.key_path,
        cert_path = config.cert_path,
        enable_auth = config.enable_auth,
        privkey_path = config.privkey_path,
        pubkey_path = config.pubkey_path,
        hash = config.hash,
        expiry = config.expiry
    )
}

fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("couldn't get path to executable");
    cfgpath.set_file_name(versions::CONFIG_NAME);

    let Ok(toml_str) = std::fs::read_to_string(&cfgpath) else {
        let config = Config::default();
        write_config_file(&cfgpath, &config);
        return config;
    };

    let toml = toml_str
        .parse::<DocumentMut>()
        .expect("config file is invalid");

    let (migration_occured, config) = migrate_config(toml);

    if migration_occured {
        write_config_file(&cfgpath, &config);
    }

    config
}
