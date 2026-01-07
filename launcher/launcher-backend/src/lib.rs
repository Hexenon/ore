//! Backend orchestration for launcher services.

pub mod actions;
pub mod config;
pub mod error;
pub mod scheduler;
pub mod wallet;

pub use actions::{AdminAction, AdminActionPayload, ActionSigner, SignedAdminAction};
pub use config::{ActionPolicyConfig, BackendConfig, ScheduleConfig, WalletConfig};
pub use error::BackendError;
pub use scheduler::{ActionExecutor, ActionScheduler, OnChainState, OnChainStateProvider};

#[derive(Debug, Clone, Copy)]
pub struct BackendStatus {
    pub healthy: bool,
}

impl BackendStatus {
    pub fn ok() -> Self {
        Self { healthy: true }
    }
}
