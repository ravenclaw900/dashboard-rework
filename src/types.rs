use serde::Serialize;
use tsync_macro::tsync;

#[derive(Serialize, Clone)]
#[tsync]
pub struct UsageData {
    pub used: u64,
    pub total: u64,
    pub percent: f32,
}

#[derive(Serialize)]
#[tsync]
pub struct SystemData {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
}
