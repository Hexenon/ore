//! Backend orchestration for launcher services.

pub mod actions;
pub mod config;
pub mod error;
pub mod launch;
pub mod scheduler;
pub mod service;
pub mod wallet;

pub use actions::{AdminAction, AdminActionPayload, ActionSigner, SignedAdminAction};
pub use config::{ActionPolicyConfig, BackendConfig, ScheduleConfig, WalletConfig};
pub use error::BackendError;
pub use launch::{execute_launch, LaunchPlan, LaunchResult, LpPoolPlan, MintPlan, ProgramIdsPlan, VaultPlan};
pub use scheduler::{ActionExecutor, ActionScheduler, OnChainState, OnChainStateProvider};
pub use service::{LaunchService, LaunchStore};

#[derive(Debug, Clone, Copy)]
pub struct BackendStatus {
    pub healthy: bool,
}

impl BackendStatus {
    pub fn ok() -> Self {
        Self { healthy: true }
    }
}
