use serde::Serialize;
use tsync_macro::tsync;

// Required for tsync to export data (copied from pretty_bytes_typed)
#[allow(dead_code)]
mod tsync {
    use tsync_macro::tsync;

    #[tsync]
    enum BinarySuffix {
        B,
        KiB,
        MiB,
        GiB,
        TiB,
        PiB,
        // Technically more suffixes, but if someone has this much memory they have other problems
    }

    #[tsync]
    struct PrettyBytesBinary {
        num: f64,
        suffix: BinarySuffix,
    }
}

#[derive(Serialize, Clone)]
#[tsync]
pub struct UsageData {
    pub used: pretty_bytes_typed::PrettyBytesBinary,
    pub total: pretty_bytes_typed::PrettyBytesBinary,
    pub percent: f32,
}

#[derive(Serialize)]
#[tsync]
pub struct System {
    pub cpu: f32,
    pub ram: UsageData,
    pub swap: UsageData,
}
