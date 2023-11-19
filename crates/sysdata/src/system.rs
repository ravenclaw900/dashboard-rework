use sysinfo::{System, SystemExt};
use tokio::sync::{mpsc, oneshot};

use super::getters;
use super::system_cache::{try_from_cache_or_init, SystemCache};
use super::types;

pub type RequestTx = mpsc::Sender<Request>;

pub enum Request {
    System(oneshot::Sender<types::SystemData>),
}

pub fn spawn_system_task() -> RequestTx {
    // 10 is probably a reasonable buffer size
    let (tx, mut rx) = mpsc::channel(10);

    let mut sys = System::new();
    let mut cache = SystemCache::new();

    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            match req {
                Request::System(channel) => {
                    let cpu = try_from_cache_or_init(&mut cache.cpu, &mut sys, getters::cpu);
                    let mem = try_from_cache_or_init(&mut cache.memory, &mut sys, getters::memory);

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
