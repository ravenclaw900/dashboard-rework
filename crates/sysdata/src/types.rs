#[derive(Clone)]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

pub struct SystemData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
}
