use std::time::Duration;

use ephemeropt::EphemeralOption;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::{mpsc, oneshot};

pub enum SystemRequest {
    Cpu(oneshot::Sender<f32>),
}

struct SystemDataCache {
    cpu: EphemeralOption<f32>,
}

impl SystemDataCache {
    const fn new() -> Self {
        Self {
            cpu: EphemeralOption::new_empty(Duration::from_millis(900)),
        }
    }
}

pub fn system_task() -> mpsc::Sender<SystemRequest> {
    // 10 is probably a reasonable buffer size
    let (tx, mut rx) = mpsc::channel(10);

    let mut sys = System::new();
    let mut cache = SystemDataCache::new();

    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            match req {
                SystemRequest::Cpu(channel) => {
                    let cpu = if let Some(&cpu) = cache.cpu.get() {
                        cpu
                    } else {
                        let cpu = cpu(&mut sys);
                        cache.cpu.insert(cpu);
                        cpu
                    };
                    // Ignore channel send result
                    let _ = channel.send(cpu);
                }
            }
        }
    });

    tx
}

pub fn cpu(sys: &mut System) -> f32 {
    sys.refresh_cpu();
    round_percent(sys.global_cpu_info().cpu_usage())
}

fn round_percent(val: f32) -> f32 {
    (val * 100.).round() / 100.
}
