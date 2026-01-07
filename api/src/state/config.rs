use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::config_pda;

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

    /// Basis points of the round reward allocated to the motherlode.
    pub motherlode_bps: u64,

    /// Basis points of buried ORE shared with stakers.
    pub stake_bps: u64,

    /// Reserved for future config fields.
    pub reserved: [u8; 32],
}

impl Config {
    pub fn pda(mint: Pubkey) -> (Pubkey, u8) {
        config_pda(mint)
    }
}

account!(OreAccount, Config);
