use ephemeropt::EphemeralOption;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use tokio::sync::{mpsc, oneshot};

use crate::types;

use super::getters;

const CACHE_DURATION: Duration = Duration::from_millis(1500);

pub type RequestTx = mpsc::Sender<Request>;

pub enum Request {
    System(oneshot::Sender<types::SystemData>),
}

struct SystemDataCache {
    cpu: EphemeralOption<f32>,
    memory: EphemeralOption<(types::UsageData, types::UsageData)>,
}

impl SystemDataCache {
    const fn new() -> Self {
        Self {
            cpu: EphemeralOption::new_empty(CACHE_DURATION),
            memory: EphemeralOption::new_empty(CACHE_DURATION),
        }
    }
}

fn try_from_cache<T, F>(cache: &mut EphemeralOption<T>, sys: &mut System, init_fn: F) -> T
where
    F: FnOnce(&mut System) -> T,
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

pub fn spawn_system_task() -> mpsc::Sender<Request> {
    // 10 is probably a reasonable buffer size
    let (tx, mut rx) = mpsc::channel(10);

    let mut sys = System::new();
    let mut cache = SystemDataCache::new();

    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            match req {
                Request::System(channel) => {
                    let cpu = try_from_cache(&mut cache.cpu, &mut sys, getters::cpu);
                    let mem = try_from_cache(&mut cache.memory, &mut sys, getters::memory);

                    let sysdata = types::SystemData {
                        cpu,
                        ram: mem.0,
                        swap: mem.1,
                    };

                    // Ignore channel send result
                    let _ = channel.send(sysdata);
                }
            }
        }
    });

    tx
}
