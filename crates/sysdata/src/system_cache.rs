use ephemeropt::EphemeralOption;
use std::time::Duration;
use sysinfo::System;

use super::types;

const CACHE_DURATION: Duration = Duration::from_millis(1500);

pub struct SystemCache {
    pub system: EphemeralOption<types::SystemData>,
    pub processes: EphemeralOption<Vec<types::ProcessData>>,
}

impl SystemCache {
    pub const fn new() -> Self {
        Self {
            system: EphemeralOption::new_empty(CACHE_DURATION),
            processes: EphemeralOption::new_empty(CACHE_DURATION),
        }
    }
}

pub fn try_from_cache_or_init<T>(
    cache: &mut EphemeralOption<T>,
    sys: &mut System,
    init_fn: fn(&mut System) -> T,
) -> T
where
    T: Clone,
{
    // Use if let else here to satisfy borrow checker
    #[allow(clippy::option_if_let_else)]
    if let Some(val) = cache.get() {
        val.clone()
    } else {
        let val = init_fn(sys);
        cache.insert(val.clone());
        val
    }
}
