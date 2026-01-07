//! API types for mining services.

#[derive(Debug, Clone)]
pub struct MiningJob {
    pub id: String,
    pub target_hashrate: u64,
}
