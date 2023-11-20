use once_cell::sync::Lazy;
use serde::Deserialize;
use toml_edit::{value, Document};

pub static CONFIG: Lazy<Config> = Lazy::new(config);

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
}

fn migrate(doc: &mut Document) -> bool {
    let toml = doc.as_table();
    let version = toml
        .get("CONFIG_VERSION_DO_NOT_CHANGE")
        .map(|x| x.as_integer().unwrap());
    match version {
        // From the original version of the dashboard
        // Not even worth doing a migration, just try and copy old settings over to a file with new "defaults"
        None => {
            let mut new_toml = Document::new();
            new_toml["CONFIG_VERSION_DO_NOT_CHANGE"] = value(1);
            new_toml["port"] = value(5252);

            if let Some(port) = toml.get("port") {
                new_toml["port"] = port.clone();
            }

            *doc = new_toml;
            false
        }
        Some(_) => true,
    }
}

fn config() -> Config {
    let mut cfgpath = std::env::current_exe().expect("couldn't get path to executable");
    cfgpath.set_file_name("config.toml");

    let toml_str = std::fs::read_to_string(&cfgpath).expect("couldn't read config file");
    let mut toml = toml_str
        .parse::<Document>()
        .expect("config file is invalid");

    loop {
        let migration_done = migrate(&mut toml);
        if migration_done {
            break;
        } else {
            std::fs::write(&cfgpath, toml.to_string().as_bytes())
                .expect("failed to migrate config file");
        }
    }

    toml_edit::de::from_document(toml).expect("failed to parse config file")
}
