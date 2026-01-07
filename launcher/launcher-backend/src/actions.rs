use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::signer::keypair::Keypair;

use crate::error::BackendError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdminAction {
    Reset,
    Buyback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionPayload {
    pub action: AdminAction,
    pub slot: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct SignedAdminAction {
    pub action: AdminAction,
    pub slot: u64,
    pub timestamp: u64,
    pub signature: Signature,
    pub signer: solana_sdk::pubkey::Pubkey,
}

pub struct ActionSigner {
    keypair: Keypair,
}

impl ActionSigner {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub fn pubkey(&self) -> solana_sdk::pubkey::Pubkey {
        self.keypair.pubkey()
    }

    pub fn sign_action(&self, payload: &AdminActionPayload) -> Result<SignedAdminAction, BackendError> {
        let message = bincode::serialize(payload)
            .map_err(|err| BackendError::SigningFailed(err.to_string()))?;
        let signature = self
            .keypair
            .sign_message(&message);
        Ok(SignedAdminAction {
            action: payload.action,
            slot: payload.slot,
            timestamp: payload.timestamp,
            signature,
            signer: self.keypair.pubkey(),
        })
    }
}
