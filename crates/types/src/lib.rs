#[derive(Clone)]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

#[derive(Clone)]
pub struct SystemData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
}

#[derive(Clone)]
pub struct ProcessData {
    pub pid: usize,
    pub mem: u64,
    pub cpu: f32,
    pub status: String,
    pub name: String,
    pub runtime: u64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessSignal {
    Kill,
    Term,
    Stop,
    Resume,
}

#[derive(Clone, Debug)]
pub struct HostData {
    pub hostname: String,
    pub net_interface: String,
    pub ip_addr: String,
    pub dietpi_version: String,
    pub system_version: String,
    pub arch: String,
    pub installed_packages: u32,
    pub upgradable_packages: u32,
}
