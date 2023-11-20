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
    pub runtime: std::time::Duration,
}
