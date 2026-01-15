//! Reward locking policies and program logic.

mod instruction;
mod processor;
mod state;

pub use instruction::RewardsLockInstruction;
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
