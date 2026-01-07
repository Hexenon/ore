use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;

use crate::actions::{AdminAction, AdminActionPayload, ActionSigner, SignedAdminAction};
use crate::config::{ActionPolicyConfig, BackendConfig};
use crate::error::BackendError;

#[derive(Debug, Clone)]
pub struct OnChainState {
    pub slot: u64,
    pub last_reset_slot: Option<u64>,
    pub treasury_lamports: u64,
}

impl ActionPolicyConfig {
    pub fn evaluate(&self, state: &OnChainState) -> Vec<AdminAction> {
        let mut actions = Vec::new();
        if self.enable_reset {
            if let Some(min_gap) = self.reset_min_slot_gap {
                let should_reset = match state.last_reset_slot {
                    Some(last) => state.slot.saturating_sub(last) >= min_gap,
                    None => true,
                };
                if should_reset {
                    actions.push(AdminAction::Reset);
                }
            }
        }
        if self.enable_buyback {
            if let Some(min_lamports) = self.buyback_min_treasury_lamports {
                if state.treasury_lamports >= min_lamports {
                    actions.push(AdminAction::Buyback);
                }
            }
        }
        actions
    }
}

#[async_trait]
pub trait OnChainStateProvider: Send + Sync {
    async fn fetch_state(&self) -> Result<OnChainState, BackendError>;
}

#[async_trait]
pub trait ActionExecutor: Send + Sync {
    async fn handle_action(&self, action: SignedAdminAction) -> Result<(), BackendError>;
}

pub struct ActionScheduler<P, E> {
    config: BackendConfig,
    signer: ActionSigner,
    provider: P,
    executor: E,
}

impl<P, E> ActionScheduler<P, E>
where
    P: OnChainStateProvider,
    E: ActionExecutor,
{
    pub fn new(config: BackendConfig, signer: ActionSigner, provider: P, executor: E) -> Self {
        Self {
            config,
            signer,
            provider,
            executor,
        }
    }

    pub async fn run(&self) -> Result<(), BackendError> {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.schedule.poll_interval_secs));
        loop {
            interval.tick().await;
            let state = self.provider.fetch_state().await?;
            for action in self.config.policy.evaluate(&state) {
                self.trigger_action(action, &state).await?;
            }
        }
    }

    pub async fn trigger_action(
        &self,
        action: AdminAction,
        state: &OnChainState,
    ) -> Result<(), BackendError> {
        let payload = AdminActionPayload {
            action,
            slot: state.slot,
            timestamp: unix_timestamp(),
        };
        let signed = self.signer.sign_action(&payload)?;
        self.executor.handle_action(signed).await
    }
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
