use crate::types::UsageData;
use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};

pub fn cpu(sys: &mut System) -> f32 {
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    round_percent(sys.global_cpu_info().cpu_usage())
}

#[allow(clippy::cast_precision_loss)]
pub fn memory(sys: &mut System) -> (UsageData, UsageData) {
    // refresh_memory refreshes RAM and Swap, but used_memory and used_swap return RAM and Swap, respectively
    sys.refresh_memory();

    let ram_used = sys.used_memory();
    let ram_total = sys.total_memory();
    let ram_percent = round_percent(((ram_used as f32) / (ram_total as f32)) * 100.);

    let swap_used = sys.used_swap();
    let swap_total = sys.total_swap();
    let swap_percent = round_percent(((swap_used as f32) / (swap_total as f32)) * 100.);

    (
        UsageData {
            used: ram_used,
            total: ram_total,
            percent: ram_percent,
        },
        UsageData {
            used: swap_used,
            total: swap_total,
            percent: swap_percent,
        },
    )
}

fn round_percent(val: f32) -> f32 {
    (val * 100.).round() / 100.
}
