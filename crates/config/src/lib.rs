use std::{ops::Deref, path::Path, sync::OnceLock};
use toml_edit::DocumentMut;

use crate::types::Config;

mod migrate;
mod types;

pub static CONFIG: ConfigStatic = ConfigStatic(OnceLock::new());
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Basically equivalent to a LazyLock<Config>
pub struct ConfigStatic(OnceLock<Config>);

impl Deref for ConfigStatic {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.0.get_or_init(config)
    }
}

fn write_config_file(path: &Path, file_data: &str) {
    std::fs::write(path, file_data.as_bytes()).expect("failed to write new config file");
}

fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("couldn't get path to executable");
    cfgpath.set_file_name("config.toml");

    tracing::info!("Loading config file from {}", cfgpath.display());

    let Ok(toml_str) = std::fs::read_to_string(&cfgpath) else {
        tracing::warn!("No config file found, generating new one");

        let config_file = migrate::generate_config(&Config::DEFAULT);
        write_config_file(&cfgpath, &config_file);
        return Config::DEFAULT;
    };
    let toml = toml_str
        .parse::<DocumentMut>()
        .expect("config file is invalid");

    let migration = migrate::migrate(&toml);

    if let Some((config_file, config)) = migration {
        write_config_file(&cfgpath, &config_file);
        return config;
    }

    toml_edit::de::from_document(toml).expect("failed to parse config file")
}
