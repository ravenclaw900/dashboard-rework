use crate::types::UsageData;
use pretty_bytes_typed::pretty_bytes_binary;
use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};

#[derive(Clone)]
pub struct MemoryData {
    pub ram: UsageData,
    pub swap: UsageData,
}

pub fn cpu(sys: &mut System) -> f32 {
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    round_percent(sys.global_cpu_info().cpu_usage())
}

#[allow(clippy::cast_precision_loss)]
pub fn memory(sys: &mut System) -> MemoryData {
    // refresh_memory refreshes RAM and Swap, but used_memory and used_swap return RAM and Swap, respectively
    sys.refresh_memory();

    let ram_used = sys.used_memory();
    let ram_total = sys.total_memory();
    let ram_percent = round_percent((ram_used as f32) / (ram_total as f32));

    let swap_used = sys.used_swap();
    let swap_total = sys.total_swap();
    let swap_percent = round_percent((swap_used as f32) / (swap_total as f32));

    MemoryData {
        ram: UsageData {
            used: pretty_bytes_binary(ram_used as f64, Some(2)),
            total: pretty_bytes_binary(ram_total as f64, Some(2)),
            percent: ram_percent,
        },
        swap: UsageData {
            used: pretty_bytes_binary(swap_used as f64, Some(2)),
            total: pretty_bytes_binary(swap_total as f64, Some(2)),
            percent: swap_percent,
        },
    }
}

fn round_percent(val: f32) -> f32 {
    (val * 100.).round() / 100.
}
