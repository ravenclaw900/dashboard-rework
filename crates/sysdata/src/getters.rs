use sysinfo::{CpuRefreshKind, Networks, ProcessRefreshKind, System};

use crate::types;

fn cpu(sys: &mut System) -> f32 {
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    round_percent(sys.global_cpu_info().cpu_usage())
}

#[allow(clippy::cast_precision_loss)]
fn memory(sys: &mut System) -> (types::UsageData, types::UsageData) {
    // refresh_memory refreshes both RAM and Swap, but used_memory and used_swap return RAM and Swap, respectively
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
            runtime: proc.run_time(),
        })
    }

    processes
}

pub async fn host() -> types::HostData {
    use tokio::fs;
    use tokio::process::Command;

    let unknown = || "unknown".to_string();

    let mut networks = Networks::new();
    networks.refresh_list();

    let net_data = networks
        .iter()
        .find(|(k, _)| k.contains("eth") || k.contains("wlan"));
    let net_interface = net_data.map(|(k, _)| k.clone()).unwrap_or_else(unknown);
    let ip_addr = net_data
        .and_then(|(_, v)| v.ip_networks().first().map(|x| x.addr.to_string()))
        .unwrap_or_else(unknown);

    let hostname = System::host_name().unwrap_or_else(unknown);
    let arch = System::cpu_arch().unwrap_or_else(unknown);
    let system_version = System::os_version()
        .map(|ver| format!("Debian {ver}"))
        .unwrap_or_else(unknown);

    let dietpi_version = {
        let version_file = fs::read_to_string("/boot/dietpi/.version").await.ok();
        // This monstrosity takes the first 3 lines of the version file, gets the numbers after the = sign,
        // and places dots between them
        // Unwrap is used because if the file exists, it should be formatted correctly
        version_file
            .map(|x| {
                x.lines()
                    .take(3)
                    .map(|x| x.split_once('=').unwrap().1.to_string())
                    .reduce(|acc, x| format!("{acc}.{x}"))
                    .unwrap()
            })
            .unwrap_or_else(unknown)
    };

    // Counts number of newlines in a `dpkg --get-selections` command
    let installed_packages = Command::new("dpkg")
        .arg("--get-selections")
        .output()
        .await
        .map(|x| x.stdout.iter().filter(|&&x| x == b'\n').count() as u32)
        .unwrap_or(0);

    // Number of upgradable packages is stored in this file
    // Converts both results to Option to allow the use of and_then with different error types
    let upgradable_packages = fs::read_to_string("/run/dietpi/.apt_updates")
        .await
        .ok()
        .and_then(|x| x.parse::<u32>().ok())
        .unwrap_or(0);

    types::HostData {
        hostname,
        net_interface,
        ip_addr,
        dietpi_version,
        system_version,
        arch,
        installed_packages,
        upgradable_packages,
    }
}
