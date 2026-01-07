use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use steel::*;

use crate::{
    consts::{BOARD, SOL_MINT},
    instruction::*,
    state::*,
};

#[derive(Clone, Copy, Debug)]
pub struct OreSdk {
    pub mint: Pubkey,
    pub program_id: Pubkey,
}

impl OreSdk {
    pub fn new(mint: Pubkey) -> Self {
        Self {
            mint,
            program_id: crate::ID,
        }
    }

    pub fn program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = program_id;
        self
    }

    pub fn log(&self, signer: Pubkey, msg: &[u8]) -> Instruction {
        log(signer, msg)
    }

    pub fn automate(
        &self,
        signer: Pubkey,
        amount: u64,
        deposit: u64,
        executor: Pubkey,
        fee: u64,
        mask: u64,
        strategy: u8,
        reload: bool,
    ) -> Instruction {
        automate(
            self.mint, signer, amount, deposit, executor, fee, mask, strategy, reload,
        )
    }

    pub fn claim_sol(&self, signer: Pubkey) -> Instruction {
        claim_sol(self.mint, signer)
    }

    pub fn claim_ore(&self, signer: Pubkey) -> Instruction {
        claim_ore(self.mint, signer)
    }

    pub fn deploy(
        &self,
        signer: Pubkey,
        authority: Pubkey,
        amount: u64,
        round_id: u64,
        squares: [bool; 25],
    ) -> Instruction {
        deploy(self.mint, signer, authority, amount, round_id, squares)
    }

    pub fn buyback(
        &self,
        signer: Pubkey,
        swap_accounts: &[AccountMeta],
        swap_data: &[u8],
    ) -> Instruction {
        buyback(self.mint, signer, swap_accounts, swap_data)
    }

    pub fn bury(&self, signer: Pubkey, amount: u64) -> Instruction {
        bury(self.mint, signer, amount)
    }

    pub fn liq(&self, signer: Pubkey, manager: Pubkey) -> Instruction {
        liq(self.mint, signer, manager)
    }

    pub fn wrap(&self, signer: Pubkey, amount: u64) -> Instruction {
        wrap(self.mint, signer, amount)
    }

    pub fn reset(
        &self,
        signer: Pubkey,
        fee_collector: Pubkey,
        round_id: u64,
        top_miner: Pubkey,
    ) -> Instruction {
        reset(self.mint, signer, fee_collector, round_id, top_miner)
    }

    pub fn close(&self, signer: Pubkey, round_id: u64, rent_payer: Pubkey) -> Instruction {
        close(self.mint, signer, round_id, rent_payer)
    }

    pub fn checkpoint(&self, signer: Pubkey, authority: Pubkey, round_id: u64) -> Instruction {
        checkpoint(self.mint, signer, authority, round_id)
    }

    pub fn set_admin(&self, signer: Pubkey, admin: Pubkey) -> Instruction {
        set_admin(self.mint, signer, admin)
    }

    pub fn deposit(
        &self,
        signer: Pubkey,
        payer: Pubkey,
        amount: u64,
        compound_fee: u64,
    ) -> Instruction {
        deposit(self.mint, signer, payer, amount, compound_fee)
    }

    pub fn withdraw(&self, signer: Pubkey, amount: u64) -> Instruction {
        withdraw(self.mint, signer, amount)
    }

    pub fn reload_sol(&self, signer: Pubkey, authority: Pubkey) -> Instruction {
        reload_sol(self.mint, signer, authority)
    }

    pub fn claim_yield(&self, signer: Pubkey, amount: u64) -> Instruction {
        claim_yield(self.mint, signer, amount)
    }

    pub fn compound_yield(&self, signer: Pubkey) -> Instruction {
        compound_yield(self.mint, signer)
    }

    pub fn new_var(
        &self,
        signer: Pubkey,
        provider: Pubkey,
        id: u64,
        commit: [u8; 32],
        samples: u64,
    ) -> Instruction {
        new_var(self.mint, signer, provider, id, commit, samples)
    }
}

pub struct OreSdkBuilder {
    mint: Pubkey,
    program_id: Pubkey,
}

impl OreSdkBuilder {
    pub fn new(mint: Pubkey) -> Self {
        Self {
            mint,
            program_id: crate::ID,
        }
    }

    pub fn program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = program_id;
        self
    }

    pub fn build(self) -> OreSdk {
        OreSdk {
            mint: self.mint,
            program_id: self.program_id,
        }
    }
}

pub fn log(signer: Pubkey, msg: &[u8]) -> Instruction {
    let mut data = Log {}.to_bytes();
    data.extend_from_slice(msg);
    Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new(signer, true)],
        data: data,
    }
}

pub fn program_log(mint: Pubkey, accounts: &[AccountInfo], msg: &[u8]) -> Result<(), ProgramError> {
    invoke_signed(
        &log(*accounts[0].key, msg),
        accounts,
        &crate::ID,
        &[BOARD, &mint.to_bytes()],
    )
}

// let [signer_info, config_info, automation_info, executor_info, miner_info, system_program] = accounts else {

pub fn automate(
    mint: Pubkey,
    signer: Pubkey,
    amount: u64,
    deposit: u64,
    executor: Pubkey,
    fee: u64,
    mask: u64,
    strategy: u8,
    reload: bool,
) -> Instruction {
    let config_address = config_pda(mint).0;
    let automation_address = automation_pda(mint, signer).0;
    let miner_address = miner_pda(mint, signer).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(automation_address, false),
            AccountMeta::new(executor, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Automate {
            amount: amount.to_le_bytes(),
            deposit: deposit.to_le_bytes(),
            fee: fee.to_le_bytes(),
            mask: mask.to_le_bytes(),
            strategy: strategy as u8,
            reload: (reload as u64).to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn claim_sol(mint: Pubkey, signer: Pubkey) -> Instruction {
    let config_address = config_pda(mint).0;
    let miner_address = miner_pda(mint, signer).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: ClaimSOL {}.to_bytes(),
    }
}

// let [signer_info, config_info, miner_info, mint_info, recipient_info, treasury_info, treasury_tokens_info, system_program, token_program, associated_token_program] =

pub fn claim_ore(mint: Pubkey, signer: Pubkey) -> Instruction {
    let config_address = config_pda(mint).0;
    let miner_address = miner_pda(mint, signer).0;
    let treasury_address = treasury_pda(mint).0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &mint);
    let recipient_address = get_associated_token_address(&signer, &mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: ClaimORE {}.to_bytes(),
    }
}

// let [signer_info, authority_info, automation_info, board_info, config_info, miner_info, round_info, system_program] =

pub fn deploy(
    mint: Pubkey,
    signer: Pubkey,
    authority: Pubkey,
    amount: u64,
    round_id: u64,
    squares: [bool; 25],
) -> Instruction {
    let automation_address = automation_pda(mint, authority).0;
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let miner_address = miner_pda(mint, authority).0;
    let round_address = round_pda(mint, round_id).0;
    let entropy_var_address = entropy_api::state::var_pda(board_address, 0).0;

    // Convert array of 25 booleans into a 32-bit mask where each bit represents whether
    // that square index is selected (1) or not (0)
    let mut mask: u32 = 0;
    for (i, &square) in squares.iter().enumerate() {
        if square {
            mask |= 1 << i;
        }
    }

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(automation_address, false),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new(round_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(crate::ID, false),
            // Entropy accounts.
            AccountMeta::new(entropy_var_address, false),
            AccountMeta::new_readonly(entropy_api::ID, false),
        ],
        data: Deploy {
            amount: amount.to_le_bytes(),
            squares: mask.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// let [pool, user_source_token, user_destination_token, a_vault, b_vault, a_token_vault, b_token_vault, a_vault_lp_mint, b_vault_lp_mint, a_vault_lp, b_vault_lp, protocol_token_fee, user_key, vault_program, token_program] =

pub fn buyback(
    mint: Pubkey,
    signer: Pubkey,
    swap_accounts: &[AccountMeta],
    swap_data: &[u8],
) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let treasury_address = treasury_pda(mint).0;
    let treasury_ore_address = get_associated_token_address(&treasury_address, &mint);
    let treasury_sol_address = get_associated_token_address(&treasury_address, &SOL_MINT);
    let mut accounts = vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(board_address, false),
        AccountMeta::new_readonly(config_address, false),
        AccountMeta::new(mint, false),
        AccountMeta::new(treasury_address, false),
        AccountMeta::new(treasury_ore_address, false),
        AccountMeta::new(treasury_sol_address, false),
        AccountMeta::new_readonly(spl_token::ID, false),
        AccountMeta::new_readonly(crate::ID, false),
    ];
    for account in swap_accounts.iter() {
        let mut acc_clone = account.clone();
        acc_clone.is_signer = false;
        accounts.push(acc_clone);
    }
    let mut data = Buyback {}.to_bytes();
    data.extend_from_slice(swap_data);
    Instruction {
        program_id: crate::ID,
        accounts,
        data,
    }
}

// let [signer_info, sender_info, board_info, config_info, mint_info, treasury_info, treasury_ore_info, token_program, ore_program] =

pub fn bury(mint: Pubkey, signer: Pubkey, amount: u64) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let sender_address = get_associated_token_address(&signer, &mint);
    let treasury_address = treasury_pda(mint).0;
    let treasury_ore_address = get_associated_token_address(&treasury_address, &mint);
    let token_program = spl_token::ID;
    let ore_program = crate::ID;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_ore_address, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(ore_program, false),
        ],
        data: Bury {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// let [signer_info, board_info, config_info, manager_info, manager_sol_info, treasury_info, treasury_sol_info, token_program, ore_program] =

pub fn liq(mint: Pubkey, signer: Pubkey, manager: Pubkey) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let manager_sol_address = get_associated_token_address(&manager, &SOL_MINT);
    let treasury_address = treasury_pda(mint).0;
    let treasury_sol_address = get_associated_token_address(&treasury_address, &SOL_MINT);
    let token_program = spl_token::ID;
    let ore_program = crate::ID;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(manager, false),
            AccountMeta::new(manager_sol_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_sol_address, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(ore_program, false),
        ],
        data: Liq {}.to_bytes(),
    }
}

pub fn wrap(mint: Pubkey, signer: Pubkey, amount: u64) -> Instruction {
    let config_address = config_pda(mint).0;
    let treasury_address = treasury_pda(mint).0;
    let treasury_sol_address = get_associated_token_address(&treasury_address, &SOL_MINT);
    Instruction {
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_sol_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        program_id: crate::ID,
        data: Wrap {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// let [signer_info, board_info, config_info, fee_collector_info, mint_info, round_info, round_next_info, top_miner_info, treasury_info, treasury_tokens_info, system_program, token_program, ore_program, slot_hashes_sysvar] =

pub fn reset(
    mint: Pubkey,
    signer: Pubkey,
    fee_collector: Pubkey,
    round_id: u64,
    top_miner: Pubkey,
) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let round_address = round_pda(mint, round_id).0;
    let round_next_address = round_pda(mint, round_id + 1).0;
    let top_miner_address = miner_pda(mint, top_miner).0;
    let treasury_address = treasury_pda(mint).0;
    let treasury_tokens_address = treasury_tokens_address(mint);
    let entropy_var_address = entropy_api::state::var_pda(board_address, 0).0;
    let mint_authority_address = ore_mint_api::state::authority_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(fee_collector, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(round_address, false),
            AccountMeta::new(round_next_address, false),
            AccountMeta::new(top_miner_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(crate::ID, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
            // Entropy accounts.
            AccountMeta::new(entropy_var_address, false),
            AccountMeta::new_readonly(entropy_api::ID, false),
            // Mint accounts.
            AccountMeta::new(mint_authority_address, false),
            AccountMeta::new_readonly(ore_mint_api::ID, false),
        ],
        data: Reset {}.to_bytes(),
    }
}

// let [signer_info, board_info, rent_payer_info, round_info, treasury_info, system_program] =

pub fn close(mint: Pubkey, signer: Pubkey, round_id: u64, rent_payer: Pubkey) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let treasury_address = treasury_pda(mint).0;
    let round_address = round_pda(mint, round_id).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(rent_payer, false),
            AccountMeta::new(round_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Close {}.to_bytes(),
    }
}

// let [signer_info, config_info, board_info, miner_info, round_info, treasury_info, system_program] =

pub fn checkpoint(mint: Pubkey, signer: Pubkey, authority: Pubkey, round_id: u64) -> Instruction {
    let config_address = config_pda(mint).0;
    let miner_address = miner_pda(mint, authority).0;
    let board_address = board_pda(mint).0;
    let round_address = round_pda(mint, round_id).0;
    let treasury_address = treasury_pda(mint).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(board_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new(round_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Checkpoint {}.to_bytes(),
    }
}

pub fn set_admin(mint: Pubkey, signer: Pubkey, admin: Pubkey) -> Instruction {
    let config_address = config_pda(mint).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: SetAdmin {
            admin: admin.to_bytes(),
        }
        .to_bytes(),
    }
}

// let [signer_info, payer_info, config_info, mint_info, sender_info, stake_info, stake_tokens_info, treasury_info, system_program, token_program, associated_token_program] =

pub fn deposit(
    mint: Pubkey,
    signer: Pubkey,
    payer: Pubkey,
    amount: u64,
    compound_fee: u64,
) -> Instruction {
    let config_address = config_pda(mint).0;
    let stake_address = stake_pda(mint, signer).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &mint);
    let sender_address = get_associated_token_address(&signer, &mint);
    let treasury_address = treasury_pda(mint).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Deposit {
            amount: amount.to_le_bytes(),
            compound_fee: compound_fee.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// let [signer_info, config_info, mint_info, recipient_info, stake_info, stake_tokens_info, treasury_info, system_program, token_program, associated_token_program] =

pub fn withdraw(mint: Pubkey, signer: Pubkey, amount: u64) -> Instruction {
    let config_address = config_pda(mint).0;
    let stake_address = stake_pda(mint, signer).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &mint);
    let recipient_address = get_associated_token_address(&signer, &mint);
    let treasury_address = treasury_pda(mint).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Withdraw {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

// let [signer_info, config_info, automation_info, miner_info, system_program] = accounts else {

pub fn reload_sol(mint: Pubkey, signer: Pubkey, authority: Pubkey) -> Instruction {
    let config_address = config_pda(mint).0;
    let automation_address = automation_pda(mint, authority).0;
    let miner_address = miner_pda(mint, authority).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(automation_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: ReloadSOL {}.to_bytes(),
    }
}

// let [signer_info, config_info, mint_info, recipient_info, stake_info, treasury_info, treasury_tokens_info, system_program, token_program, associated_token_program] =

pub fn claim_yield(mint: Pubkey, signer: Pubkey, amount: u64) -> Instruction {
    let config_address = config_pda(mint).0;
    let stake_address = stake_pda(mint, signer).0;
    let recipient_address = get_associated_token_address(&signer, &mint);
    let treasury_address = treasury_pda(mint).0;
    let treasury_tokens_address = treasury_tokens_address(mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: ClaimYield {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

pub fn compound_yield(mint: Pubkey, signer: Pubkey) -> Instruction {
    let config_address = config_pda(mint).0;
    let stake_address = stake_pda(mint, signer).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &mint);
    let treasury_address = treasury_pda(mint).0;
    let treasury_tokens_address = treasury_tokens_address(mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: CompoundYield {}.to_bytes(),
    }
}

pub fn new_var(
    mint: Pubkey,
    signer: Pubkey,
    provider: Pubkey,
    id: u64,
    commit: [u8; 32],
    samples: u64,
) -> Instruction {
    let board_address = board_pda(mint).0;
    let config_address = config_pda(mint).0;
    let var_address = entropy_api::state::var_pda(board_address, id).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(board_address, false),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(provider, false),
            AccountMeta::new(var_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(entropy_api::ID, false),
        ],
        data: NewVar {
            id: id.to_le_bytes(),
            commit: commit,
            samples: samples.to_le_bytes(),
        }
        .to_bytes(),
    }
}
