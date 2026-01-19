//! Reward locking policies and program logic.

mod instruction;
pub mod pda;
mod processor;
mod state;

pub use instruction::RewardsLockInstruction;
pub use pda::{lp_pool_pda, vault_pda, vault_schedule_hash};
pub use state::{VaultAccount, VaultImplementation, VaultSchedule, VaultState};

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, data)
}

entrypoint!(process_instruction);
