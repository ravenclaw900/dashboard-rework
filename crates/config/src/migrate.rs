use toml_edit::{DocumentMut, Item};
use tracing::level_filters::LevelFilter;

use crate::types::Config;

const LATEST_CONFIG_VERSION: i64 = 1;

macro_rules! config_template {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/config-versions/config.template.toml"
        ))
    };
}

fn wrap_string(string: &str) -> String {
    format!("\"{string}\"")
}

pub fn generate_config(config: &Config) -> String {
    format!(
        config_template!(),
        port = config.port,
        log_level = wrap_string(&config.log_level.to_string()),
        enable_tls = config.tls.enable_tls,
        key_path = wrap_string(&config.tls.key_path),
        cert_path = wrap_string(&config.tls.cert_path),
        enable_auth = config.auth.enable_auth,
        privkey_path = wrap_string(&config.auth.privkey_path),
        pubkey_path = wrap_string(&config.auth.pubkey_path),
        hash = wrap_string(&config.auth.hash),
        expiry = config.auth.expiry
    )
}

pub fn migrate(doc: &DocumentMut) -> Option<(String, Config)> {
    let version = doc
        .get("CONFIG_VERSION_DO_NOT_CHANGE")
        .and_then(Item::as_integer)
        .unwrap_or(0);

    if version == LATEST_CONFIG_VERSION {
        return None;
    }

    let mut config = Config::DEFAULT;

    tracing::info!("Fonud config version {version}, migrating to version {LATEST_CONFIG_VERSION}");

    match version {
        0 => migrate_0(doc, &mut config),
        _ => panic!("Invalid config version {version}"),
    }

    let config_file = generate_config(&config);

    Some((config_file, config))
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn migrate_0(doc: &DocumentMut, config: &mut Config) {
    if let Some(port) = doc.get("port").and_then(Item::as_integer) {
        config.port = port as u16;
    }
    if let Some(log_level) = doc
        .get("log_level")
        .and_then(Item::as_str)
        .and_then(|x| x.parse::<LevelFilter>().ok())
    {
        config.log_level = log_level;
    }

    if let Some(tls) = doc.get("tls").and_then(Item::as_bool) {
        config.tls.enable_tls = tls;
    }
    if let Some(cert) = doc.get("cert").and_then(Item::as_str) {
        config.tls.cert_path = cert.to_string();
    }
    if let Some(key) = doc.get("key").and_then(Item::as_str) {
        config.tls.key_path = key.to_string();
    }

    if let Some(pass) = doc.get("pass").and_then(Item::as_bool) {
        config.auth.enable_auth = pass;
    }
    if let Some(hash) = doc.get("hash").and_then(Item::as_str) {
        config.auth.hash = hash.to_string();
    }
    if let Some(expiry) = doc.get("expiry").and_then(Item::as_integer) {
        config.auth.expiry = expiry as u64;
    }
}
