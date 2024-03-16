use sysinfo::System;
use tokio::sync::{mpsc, oneshot};

use crate::actions;
use crate::getters;
use crate::system_cache::{try_from_cache_or_init, SystemCache};
use crate::types;

pub type RequestTx = mpsc::Sender<Request>;

pub enum Request {
    System(oneshot::Sender<types::SystemData>),
    Process(oneshot::Sender<Vec<types::ProcessData>>),
    ProcessSignal(usize, types::ProcessSignal),
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
                    let sysdata =
                        try_from_cache_or_init(&mut cache.system, &mut sys, getters::system);

                    // Ignore channel send result
                    let _ = channel.send(sysdata);
                }
                Request::Process(channel) => {
                    let processes =
                        try_from_cache_or_init(&mut cache.processes, &mut sys, getters::process);

                    // Ignore channel send result
                    let _ = channel.send(processes);
                }
                Request::ProcessSignal(pid, signal) => {
                    actions::process_signal(&mut sys, pid, signal)
                }
            }
        }
    });

    tx
}
