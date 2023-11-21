use toml_edit::{table, value, Document, Table};

pub fn migrate(doc: &mut Document) -> bool {
    let toml = doc.as_table();
    let version = toml
        .get("CONFIG_VERSION_DO_NOT_CHANGE")
        .map(|x| x.as_integer().unwrap());
    match version {
        // From the original version of the dashboard
        // Not even worth doing a migration, just try and copy old settings over to a file with new defaults
        None => {
            let new_toml = migrate_0_to_1(toml);
            *doc = new_toml;
            false
        }
        Some(_) => true,
    }
}

fn migrate_0_to_1(toml: &Table) -> Document {
    let mut new_toml = Document::new();
    new_toml["CONFIG_VERSION_DO_NOT_CHANGE"] = value(1);
    new_toml["port"] = value(5252);

    let mut tls = table();
    tls["enable_tls"] = value(false);
    tls["cert_path"] = value("");
    tls["key_path"] = value("");
    new_toml["tls"] = tls;

    let mut auth = table();
    auth["enable_auth"] = value(false);
    auth["privkey_path"] = value("");
    auth["pubkey_path"] = value("");
    auth["expiry"] = value(3600);
    new_toml["auth"] = auth;

    if let Some(port) = toml.get("port") {
        new_toml["port"] = port.clone();
    }

    if let Some(enable_tls) = toml.get("tls") {
        new_toml["tls"]["enable_tls"] = enable_tls.clone();
    }
    if let Some(cert_path) = toml.get("cert") {
        new_toml["tls"]["cert_path"] = cert_path.clone();
    }
    if let Some(key_path) = toml.get("key") {
        new_toml["tls"]["key_path"] = key_path.clone();
    }

    new_toml
}
