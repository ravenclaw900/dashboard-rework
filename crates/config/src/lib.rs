use std::{ops::Deref, sync::OnceLock};

use crate::types::Config;
use toml_edit::DocumentMut;

mod migrate;
mod types;

pub static CONFIG: ConfigStatic = ConfigStatic(OnceLock::new());

// Basically equivalent to a LazyLock<Config>
pub struct ConfigStatic(OnceLock<Config>);

impl Deref for ConfigStatic {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.0.get_or_init(config)
    }
}

fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("couldn't get path to executable");
    cfgpath.set_file_name("config.toml");

    let toml_str = std::fs::read_to_string(&cfgpath).unwrap_or_else(|_| String::new());
    let mut toml = toml_str
        .parse::<DocumentMut>()
        .expect("config file is invalid");

    let migration_occured = migrate::migrate(&mut toml);
    if migration_occured {
        std::fs::write(&cfgpath, toml.to_string().as_bytes())
            .expect("failed to migrate config file");
    }

    toml_edit::de::from_document(toml).expect("failed to parse config file")
}
