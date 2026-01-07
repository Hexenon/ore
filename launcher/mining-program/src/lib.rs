//! Mining program logic.

pub fn estimate_hashrate(cores: u16) -> u64 {
    u64::from(cores) * 1_000
}
