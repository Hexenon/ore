mod automation;
mod board;
mod config;
mod miner;
mod round;
mod stake;
mod treasury;

pub use automation::*;
pub use board::*;
pub use config::*;
pub use miner::*;
pub use round::*;
pub use stake::*;
pub use treasury::*;

use crate::consts::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum OreAccount {
    Automation = 100,
    Config = 101,
    Miner = 103,
    Treasury = 104,
    Board = 105,
    Stake = 108,
    Round = 109,
}

pub fn automation_pda(mint: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[AUTOMATION, &mint.to_bytes(), &authority.to_bytes()],
        &crate::ID,
    )
}

pub fn board_pda(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BOARD, &mint.to_bytes()], &crate::ID)
}

pub fn config_pda(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG, &mint.to_bytes()], &crate::ID)
}

pub fn miner_pda(mint: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[MINER, &mint.to_bytes(), &authority.to_bytes()],
        &crate::ID,
    )
}

pub fn round_pda(mint: Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ROUND, &mint.to_bytes(), &id.to_le_bytes()], &crate::ID)
}

pub fn stake_pda(mint: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[STAKE, &mint.to_bytes(), &authority.to_bytes()],
        &crate::ID,
    )
}

pub fn treasury_pda(mint: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[TREASURY, &mint.to_bytes()], &crate::ID)
}

pub fn treasury_tokens_address(mint: Pubkey) -> Pubkey {
    let treasury_address = treasury_pda(mint).0;
    spl_associated_token_account::get_associated_token_address(&treasury_address, &mint)
}
