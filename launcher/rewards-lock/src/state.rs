use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Supported vault implementations for reward locking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultImplementation {
    /// Custom vault logic owned by the launcher workflow.
    Custom,
}

/// A linear unlock schedule with an optional cliff and periodic releases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct VaultSchedule {
    pub start_ts: i64,
    pub cliff_ts: Option<i64>,
    pub period_seconds: i64,
    pub release_per_period: u64,
    pub period_count: u64,
}

impl VaultSchedule {
    pub fn total_amount(&self) -> u64 {
        self.release_per_period.saturating_mul(self.period_count)
    }

    pub fn released_amount(&self, now_ts: i64) -> u64 {
        if now_ts < self.start_ts {
            return 0;
        }
        if let Some(cliff_ts) = self.cliff_ts {
            if now_ts < cliff_ts {
                return 0;
            }
        }
        if self.period_seconds <= 0 {
            return self.total_amount();
        }
        let elapsed_seconds = now_ts.saturating_sub(self.start_ts);
        let periods_elapsed = (elapsed_seconds / self.period_seconds) as u64;
        let periods_vested = periods_elapsed.min(self.period_count);
        self.release_per_period.saturating_mul(periods_vested)
    }

    pub fn claimable_amount(&self, now_ts: i64, already_claimed: u64) -> u64 {
        self.released_amount(now_ts)
            .saturating_sub(already_claimed)
    }
}

/// Tracks rewards locked in a vault with a defined release schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultAccount {
    pub implementation: VaultImplementation,
    pub beneficiary: String,
    pub schedule: VaultSchedule,
    pub claimed_amount: u64,
}

impl VaultAccount {
    pub fn new(beneficiary: impl Into<String>, schedule: VaultSchedule) -> Self {
        Self {
            implementation: VaultImplementation::Custom,
            beneficiary: beneficiary.into(),
            schedule,
            claimed_amount: 0,
        }
    }

    pub fn claimable_amount(&self, now_ts: i64) -> u64 {
        self.schedule.claimable_amount(now_ts, self.claimed_amount)
    }

    pub fn claim(&mut self, now_ts: i64) -> u64 {
        let amount = self.claimable_amount(now_ts);
        self.claimed_amount = self.claimed_amount.saturating_add(amount);
        amount
    }
}

/// On-chain vault account data for rewards-lock.
#[derive(Debug, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct VaultState {
    pub beneficiary: Pubkey,
    pub schedule: VaultSchedule,
    pub bump: u8,
}
