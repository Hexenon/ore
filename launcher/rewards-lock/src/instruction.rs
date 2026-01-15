use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::VaultSchedule;

#[derive(Debug, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub enum RewardsLockInstruction {
    InitializeVault {
        beneficiary: Pubkey,
        schedule: VaultSchedule,
    },
}
