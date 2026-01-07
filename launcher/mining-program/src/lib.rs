//! Mining program logic.

use rewards_lock::VaultAccount;

pub fn estimate_hashrate(cores: u16) -> u64 {
    u64::from(cores) * 1_000
}

/// Pulls claimable rewards from the vault based on its schedule.
pub fn pull_scheduled_rewards(vault: &mut VaultAccount, now_ts: i64) -> u64 {
    vault.claim(now_ts)
}
