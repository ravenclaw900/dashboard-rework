use serde::de::DeserializeOwned;
use std::any::{Any, TypeId};
use toml_edit::DocumentMut;

pub trait Migrate: From<Self::From> + DeserializeOwned + Any {
    type From: Migrate;
    const VERSION: i64;

    fn migrate_from_doc(version: i64, doc: DocumentMut) -> Self {
        if version == Self::VERSION {
            toml_edit::de::from_document(doc).expect("failed to deserialize config file")
        } else if TypeId::of::<Self>() == TypeId::of::<Self::From>() {
            panic!("Invalid config version {version}")
        } else {
            Self::From::migrate_from_doc(version, doc).into()
        }
    }
}

pub fn migrate_config(mut doc: DocumentMut) -> (bool, super::Config) {
    let version = doc
        .remove("CONFIG_VERSION_DO_NOT_CHANGE")
        .and_then(|x| x.as_integer())
        .unwrap_or(0);

    let config = super::Config::migrate_from_doc(version, doc);
    let migration_occured = version != super::Config::VERSION;
    (migration_occured, config)
}
