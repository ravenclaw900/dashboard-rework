use ephemeropt::EphemeralOption;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use tokio::sync::{mpsc, oneshot};

use super::getters;

const CACHE_DURATION: Duration = Duration::from_millis(1500);

pub enum Request {
    Cpu(oneshot::Sender<f32>),
    Memory(oneshot::Sender<getters::MemoryData>),
}

struct SystemDataCache {
    cpu: EphemeralOption<f32>,
    memory: EphemeralOption<getters::MemoryData>,
}

impl SystemDataCache {
    const fn new() -> Self {
        Self {
            cpu: EphemeralOption::new_empty(CACHE_DURATION),
            memory: EphemeralOption::new_empty(CACHE_DURATION),
        }
    }
}

fn send_val<T, F>(
    channel: oneshot::Sender<T>,
    cache: &mut EphemeralOption<T>,
    sys: &mut System,
    init_fn: F,
) where
    F: FnOnce(&mut System) -> T,
    T: Clone,
{
    // Use if let else here to satisfy borrow checker
    #[allow(clippy::option_if_let_else)]
    let val = if let Some(val) = cache.get() {
        val.clone()
    } else {
        let val = init_fn(sys);
        cache.insert(val.clone());
        val
    };

    // Ignore channel send result
    let _ = channel.send(val);
}

pub fn spawn_system_task() -> mpsc::Sender<Request> {
    // 10 is probably a reasonable buffer size
    let (tx, mut rx) = mpsc::channel(10);

    let mut sys = System::new();
    let mut cache = SystemDataCache::new();

    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            match req {
                Request::Cpu(channel) => send_val(channel, &mut cache.cpu, &mut sys, getters::cpu),
                Request::Memory(channel) => {
                    send_val(channel, &mut cache.memory, &mut sys, getters::memory);
                }
            }
        }
    });

    tx
}
