use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::board_pda;

use super::OreAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Board {
    /// The current round number.
    pub round_id: u64,

    /// The slot at which the current round starts mining.
    pub start_slot: u64,

    /// The slot at which the current round ends mining.
    pub end_slot: u64,

    /// The current epoch id.
    pub epoch_id: u64,
}

impl Board {
    pub fn pda(&self, mint: Pubkey) -> (Pubkey, u8) {
        board_pda(mint)
    }
}

account!(OreAccount, Board);
