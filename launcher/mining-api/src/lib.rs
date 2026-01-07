//! API types for mining services.

use rewards_lock::{VaultAccount, VaultSchedule};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningJob {
    pub id: String,
    pub target_hashrate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchCreateRequest {
    pub user_id: String,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchCreateResponse {
    pub launch_id: String,
    pub status: LaunchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchStatusRequest {
    pub user_id: String,
    pub launch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchStatusResponse {
    pub launch_id: String,
    pub status: LaunchStatus,
    pub result: Option<LaunchResultInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResultInfo {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaunchStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}
