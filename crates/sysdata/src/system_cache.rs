use ephemeropt::EphemeralOption;
use std::time::Duration;
use sysinfo::System;

use crate::types;

const CACHE_DURATION: Duration = Duration::from_millis(1500);

pub struct SystemCache {
    pub system: EphemeralOption<types::SystemData>,
    pub processes: EphemeralOption<Vec<types::ProcessData>>,
    // Host data won't/shouldn't expire (except uptime, will deal with that soon)
    pub host: Option<types::HostData>,
}

impl SystemCache {
    pub const fn new() -> Self {
        Self {
            system: EphemeralOption::new_empty(CACHE_DURATION),
            processes: EphemeralOption::new_empty(CACHE_DURATION),
            host: None,
        }
    }
}

pub fn from_cache_or_init<T>(
    cache: &mut EphemeralOption<T>,
    sys: &mut System,
    init_fn: fn(&mut System) -> T,
) -> T
where
    T: Clone,
{
    match cache.get() {
        Some(val) => val.clone(),
        None => cache.insert(init_fn(sys)).clone(),
    }
}
