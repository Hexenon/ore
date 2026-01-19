pub mod mining {
    use ore_api::instruction::InitializeLpPool as OreInitializeLpPool;
    use solana_program::pubkey::Pubkey;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InitializeLpPool {
        pub base_mint: Pubkey,
        pub quote_mint: Pubkey,
    }

    impl InitializeLpPool {
        pub fn new(base_mint: Pubkey, quote_mint: Pubkey) -> Self {
            Self {
                base_mint,
                quote_mint,
            }
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            OreInitializeLpPool {
                base_mint: self.base_mint.to_bytes(),
                quote_mint: self.quote_mint.to_bytes(),
            }
            .to_bytes()
        }
    }
}

pub mod rewards_lock {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::pubkey::Pubkey;

    use ::rewards_lock::VaultSchedule;

    #[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
    pub enum RewardsLockInstruction {
        InitializeVault {
            beneficiary: Pubkey,
            schedule: VaultSchedule,
        },
    }

    impl RewardsLockInstruction {
        pub fn to_bytes(&self) -> Result<Vec<u8>, borsh::io::Error> {
            borsh::to_vec(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mining::InitializeLpPool;
    use super::rewards_lock::RewardsLockInstruction;
    use ::rewards_lock::VaultSchedule;
    use borsh::BorshDeserialize;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn initialize_lp_pool_data_matches_program_decoder() {
        let base_mint = Pubkey::new_unique();
        let quote_mint = Pubkey::new_unique();
        let data = InitializeLpPool::new(base_mint, quote_mint).to_bytes();
        let decoded = ore_api::instruction::InitializeLpPool::try_from_bytes(&data[1..])
            .expect("decode initialize lp pool");
        assert_eq!(decoded.base_mint, base_mint.to_bytes());
        assert_eq!(decoded.quote_mint, quote_mint.to_bytes());
    }

    #[test]
    fn initialize_vault_data_matches_program_decoder() {
        let beneficiary = Pubkey::new_unique();
        let schedule = VaultSchedule {
            start_ts: 123,
            cliff_ts: Some(456),
            period_seconds: 789,
            release_per_period: 10,
            period_count: 3,
        };
        let instruction = RewardsLockInstruction::InitializeVault {
            beneficiary,
            schedule,
        };
        let data = instruction
            .to_bytes()
            .expect("serialize initialize vault");
        let decoded = ::rewards_lock::RewardsLockInstruction::try_from_slice(&data)
            .expect("decode initialize vault");
        assert_eq!(
            decoded,
            ::rewards_lock::RewardsLockInstruction::InitializeVault {
                beneficiary,
                schedule,
            }
        );
    }
}
