use serde::{Deserialize, Serialize};
use steel::*;

use crate::{consts::DENOMINATOR_BPS, state::config_pda};

use super::OreAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Config {
    /// The address that can update the config.
    pub admin: Pubkey,

    /// The mint for this protocol instance.
    pub mint: Pubkey,

    /// The total amount of ORE minted per round.
    pub reward_per_round: u64,

    /// The max supply for the ORE token.
    pub max_supply: u64,

    /// Basis points of the round reward allocated to the motherlode.
    pub motherlode_bps: u64,

    /// Basis points of buried ORE shared with stakers.
    pub stake_bps: u64,

    /// Reserved for future config fields.
    pub reserved: [u8; 24],
}

impl Config {
    pub fn pda(mint: Pubkey) -> (Pubkey, u8) {
        config_pda(mint)
    }

    pub fn split_reward(&self, total_reward: u64) -> (u64, u64) {
        let motherlode_reward =
            total_reward.saturating_mul(self.motherlode_bps) / DENOMINATOR_BPS;
        let top_miner_reward = total_reward.saturating_sub(motherlode_reward);
        (top_miner_reward, motherlode_reward)
    }
}

account!(OreAccount, Config);
