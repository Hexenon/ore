use solana_program::hash::{hashv, Hash};
use solana_program::pubkey::Pubkey;

use crate::VaultSchedule;

pub const LP_POOL_SEED: &[u8] = b"lp_pool";
pub const VAULT_SEED: &[u8] = b"vault";
pub const VAULT_SCHEDULE_SEED: &[u8] = b"vault_schedule";

pub fn lp_pool_pda(mint: Pubkey, program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LP_POOL_SEED, mint.as_ref()], &program_id)
}

pub fn vault_pda(
    beneficiary: Pubkey,
    schedule: &VaultSchedule,
    program_id: Pubkey,
) -> (Pubkey, u8) {
    let schedule_hash = vault_schedule_hash(schedule);
    Pubkey::find_program_address(
        &[VAULT_SEED, beneficiary.as_ref(), schedule_hash.as_ref()],
        &program_id,
    )
}

pub fn vault_schedule_hash(schedule: &VaultSchedule) -> Hash {
    let cliff_flag: u8 = if schedule.cliff_ts.is_some() { 1 } else { 0 };
    let cliff_ts = schedule.cliff_ts.unwrap_or_default();
    hashv(&[
        VAULT_SCHEDULE_SEED,
        &schedule.start_ts.to_le_bytes(),
        &[cliff_flag],
        &cliff_ts.to_le_bytes(),
        &schedule.period_seconds.to_le_bytes(),
        &schedule.release_per_period.to_le_bytes(),
        &schedule.period_count.to_le_bytes(),
    ])
}
