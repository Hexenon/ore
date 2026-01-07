//! API types for mining services.

use rewards_lock::{VaultAccount, VaultSchedule};

#[derive(Debug, Clone)]
pub struct MiningJob {
    pub id: String,
    pub target_hashrate: u64,
}

#[derive(Debug, Clone)]
pub struct VaultInitRequest {
    pub beneficiary: String,
    pub schedule: VaultSchedule,
}

impl VaultInitRequest {
    pub fn new(beneficiary: impl Into<String>, schedule: VaultSchedule) -> Self {
        Self {
            beneficiary: beneficiary.into(),
            schedule,
        }
    }

    pub fn into_vault(self) -> VaultAccount {
        VaultAccount::new(self.beneficiary, self.schedule)
    }
}

#[derive(Debug, Clone)]
pub struct VaultClaimRequest {
    pub vault_id: String,
    pub now_ts: i64,
}

impl VaultClaimRequest {
    pub fn new(vault_id: impl Into<String>, now_ts: i64) -> Self {
        Self {
            vault_id: vault_id.into(),
            now_ts,
        }
    }
}
