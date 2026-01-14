use serde::{Deserialize, Serialize};
use steel::*;

use super::OreAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct LpPool {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub authority: Pubkey,
    pub bump: u8,
    pub reserved: [u8; 7],
}

account!(OreAccount, LpPool);
