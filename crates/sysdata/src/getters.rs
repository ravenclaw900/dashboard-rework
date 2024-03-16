use sysinfo::{CpuRefreshKind, ProcessRefreshKind, System};

use crate::types;

fn cpu(sys: &mut System) -> f32 {
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    round_percent(sys.global_cpu_info().cpu_usage())
}

#[allow(clippy::cast_precision_loss)]
fn memory(sys: &mut System) -> (types::UsageData, types::UsageData) {
    // refresh_memory refreshes RAM and Swap, but used_memory and used_swap return RAM and Swap, respectively
    sys.refresh_memory();

    let ram_used = sys.used_memory();
    let ram_total = sys.total_memory();
    let ram_percent = round_percent(((ram_used as f32) / (ram_total as f32)) * 100.);

    let swap_used = sys.used_swap();
    let swap_total = sys.total_swap();
    let swap_percent = round_percent(((swap_used as f32) / (swap_total as f32)) * 100.);

    (
        types::UsageData {
            used: ram_used,
            total: ram_total,
            percent: ram_percent,
        },
        types::UsageData {
            used: swap_used,
            total: swap_total,
            percent: swap_percent,
        },
    )
}

pub fn system(sys: &mut System) -> types::SystemData {
    let cpu = cpu(sys);
    let (ram, swap) = memory(sys);
    types::SystemData { cpu, ram, swap }
}

fn round_percent(val: f32) -> f32 {
    (val * 100.).round() / 100.
}

pub fn process(sys: &mut System) -> Vec<types::ProcessData> {
    sys.refresh_processes_specifics(
        ProcessRefreshKind::new()
            .with_cpu()
            .with_memory()
            .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet),
    );
    let process_map = sys.processes();
    let mut processes = Vec::with_capacity(process_map.len());

    // Don't put kernel threads on the list
    // (all kernel threads have an empty cmdline)
    for proc in process_map.values().filter(|x| !x.cmd().is_empty()) {
        processes.push(types::ProcessData {
            pid: proc.pid().into(),
            mem: proc.memory(),
            cpu: round_percent(proc.cpu_usage()),
            status: proc.status().to_string(),
            name: proc.name().to_string(),
            runtime: std::time::Duration::from_secs(proc.run_time()),
        })
    }

    processes
}
