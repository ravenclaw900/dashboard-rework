use toml_edit::{value, DocumentMut, Item};

const LATEST_CONFIG_VERSION: i64 = 1;

pub fn migrate(doc: &mut DocumentMut) -> bool {
    let mut version = doc
        .get("CONFIG_VERSION_DO_NOT_CHANGE")
        .and_then(Item::as_integer)
        .unwrap_or(0);

    let mut migration_occured = false;

    while version < LATEST_CONFIG_VERSION {
        match version {
            0 => migrate_0_to_1(doc),
            _ => unreachable!(),
        }

        migration_occured = true;
        version += 1;
    }

    migration_occured
}

// From the original version of the dashboard
// Not even worth doing a migration, just try and copy old settings over to a file with new defaults
fn migrate_0_to_1(old_toml: &mut DocumentMut) {
    let mut new_toml = DocumentMut::new();

    new_toml["CONFIG_VERSION_DO_NOT_CHANGE"] = value(1);
    new_toml["port"] = value(5252);

    new_toml["tls"]["enable_tls"] = value(false);
    new_toml["tls"]["cert_path"] = value("");
    new_toml["tls"]["key_path"] = value("");

    new_toml["auth"]["enable_auth"] = value(false);
    new_toml["auth"]["privkey_path"] = value("");
    new_toml["auth"]["pubkey_path"] = value("");
    new_toml["auth"]["hash"] = value("");
    new_toml["auth"]["expiry"] = value(3600);

    if let Some(port) = old_toml.get("port") {
        new_toml["port"] = port.clone();
    }

    if let Some(enable_tls) = old_toml.get("tls") {
        new_toml["tls"]["enable_tls"] = enable_tls.clone();
    }
    if let Some(cert_path) = old_toml.get("cert") {
        new_toml["tls"]["cert_path"] = cert_path.clone();
    }
    if let Some(key_path) = old_toml.get("key") {
        new_toml["tls"]["key_path"] = key_path.clone();
    }

    if let Some(enable_auth) = old_toml.get("pass") {
        new_toml["auth"]["enable_auth"] = enable_auth.clone();
    }
    if let Some(hash) = old_toml.get("hash") {
        new_toml["auth"]["hash"] = hash.clone();
    }
    if let Some(expiry) = old_toml.get("expiry") {
        new_toml["auth"]["expiry"] = expiry.clone();
    }

    *old_toml = new_toml;
}
