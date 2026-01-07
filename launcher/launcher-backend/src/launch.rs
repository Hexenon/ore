use rewards_lock::VaultSchedule;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

use crate::error::BackendError;

#[derive(Debug)]
pub struct LaunchPlan {
    pub name: Option<String>,
    pub payer: Pubkey,
    pub program_ids: ProgramIdsPlan,
    pub mint: MintPlan,
    pub lp_pool: LpPoolPlan,
    pub vaults: Vec<VaultPlan>,
}

#[derive(Debug, Clone)]
pub struct ProgramIdsPlan {
    pub ore: Pubkey,
    pub mining: Pubkey,
    pub rewards_lock: Pubkey,
}

#[derive(Debug)]
pub struct MintPlan {
    pub address: Pubkey,
    pub symbol: String,
    pub decimals: u8,
    pub authority: Option<Pubkey>,
    pub keypair: Option<Keypair>,
}

#[derive(Debug)]
pub struct LpPoolPlan {
    pub address: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub keypair: Option<Keypair>,
}

#[derive(Debug)]
pub struct VaultPlan {
    pub label: Option<String>,
    pub address: Pubkey,
    pub beneficiary: Pubkey,
    pub schedule: VaultSchedule,
    pub keypair: Option<Keypair>,
}

#[derive(Debug)]
pub struct LaunchResult {
    pub name: Option<String>,
    pub program_ids: ProgramIdsPlan,
    pub mint: MintResult,
    pub lp_pool: LpPoolResult,
    pub vaults: Vec<VaultResult>,
}

#[derive(Debug)]
pub struct MintResult {
    pub address: Pubkey,
    pub symbol: String,
    pub decimals: u8,
    pub authority: Option<Pubkey>,
    pub signature: Option<Signature>,
}

#[derive(Debug)]
pub struct LpPoolResult {
    pub address: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub signature: Option<Signature>,
}

#[derive(Debug)]
pub struct VaultResult {
    pub label: Option<String>,
    pub address: Pubkey,
    pub beneficiary: Pubkey,
    pub schedule: VaultSchedule,
    pub signature: Option<Signature>,
}

#[derive(Debug)]
pub struct LaunchInstructionSet {
    pub instructions: Vec<Instruction>,
    pub signers: Vec<Keypair>,
}

#[derive(Debug)]
pub struct MintInstructions {
    pub address: Pubkey,
    pub symbol: String,
    pub decimals: u8,
    pub authority: Option<Pubkey>,
    pub instruction_set: LaunchInstructionSet,
}

#[derive(Debug)]
pub struct LpPoolInstructions {
    pub address: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub instruction_set: LaunchInstructionSet,
}

#[derive(Debug)]
pub struct VaultInstructions {
    pub label: Option<String>,
    pub address: Pubkey,
    pub beneficiary: Pubkey,
    pub schedule: VaultSchedule,
    pub instruction_set: LaunchInstructionSet,
}

#[derive(Debug)]
pub struct LaunchInstructions {
    pub name: Option<String>,
    pub payer: Pubkey,
    pub program_ids: ProgramIdsPlan,
    pub mint: MintInstructions,
    pub lp_pool: LpPoolInstructions,
    pub vaults: Vec<VaultInstructions>,
}

pub fn build_launch_instructions(plan: LaunchPlan) -> Result<LaunchInstructions, BackendError> {
    let rent = Rent::default();
    let resolved_mint_authority = plan.mint.authority.unwrap_or(plan.payer);
    let (mint_instruction_set, mint_authority) = if let Some(mint_keypair) = plan.mint.keypair {
        let instructions = vec![
            system_instruction::create_account(
                &plan.payer,
                &plan.mint.address,
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &spl_token::ID,
            ),
            spl_token::instruction::initialize_mint2(
                &spl_token::ID,
                &plan.mint.address,
                &resolved_mint_authority,
                None,
                plan.mint.decimals,
            )
            .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))?,
        ];
        (
            LaunchInstructionSet {
                instructions,
                signers: vec![mint_keypair],
            },
            Some(resolved_mint_authority),
        )
    } else {
        (
            LaunchInstructionSet {
                instructions: Vec::new(),
                signers: Vec::new(),
            },
            plan.mint.authority,
        )
    };

    let lp_pool_instruction_set = if let Some(lp_keypair) = plan.lp_pool.keypair {
        let instructions = vec![system_instruction::create_account(
            &plan.payer,
            &plan.lp_pool.address,
            rent.minimum_balance(0),
            0,
            &plan.program_ids.mining,
        )];
        LaunchInstructionSet {
            instructions,
            signers: vec![lp_keypair],
        }
    } else {
        LaunchInstructionSet {
            instructions: Vec::new(),
            signers: Vec::new(),
        }
    };

    let mut vaults = Vec::with_capacity(plan.vaults.len());
    for vault in plan.vaults {
        let instruction_set = if let Some(vault_keypair) = vault.keypair {
            let instructions = vec![system_instruction::create_account(
                &plan.payer,
                &vault.address,
                rent.minimum_balance(0),
                0,
                &plan.program_ids.rewards_lock,
            )];
            LaunchInstructionSet {
                instructions,
                signers: vec![vault_keypair],
            }
        } else {
            LaunchInstructionSet {
                instructions: Vec::new(),
                signers: Vec::new(),
            }
        };
        vaults.push(VaultInstructions {
            label: vault.label,
            address: vault.address,
            beneficiary: vault.beneficiary,
            schedule: vault.schedule,
            instruction_set,
        });
    }

    Ok(LaunchInstructions {
        name: plan.name,
        payer: plan.payer,
        program_ids: plan.program_ids,
        mint: MintInstructions {
            address: plan.mint.address,
            symbol: plan.mint.symbol,
            decimals: plan.mint.decimals,
            authority: mint_authority,
            instruction_set: mint_instruction_set,
        },
        lp_pool: LpPoolInstructions {
            address: plan.lp_pool.address,
            base_mint: plan.lp_pool.base_mint,
            quote_mint: plan.lp_pool.quote_mint,
            instruction_set: lp_pool_instruction_set,
        },
        vaults,
    })
}

pub fn submit_launch_transactions(
    rpc_url: &str,
    payer: &Keypair,
    instructions: LaunchInstructions,
) -> Result<LaunchResult, BackendError> {
    let rpc = RpcClient::new(rpc_url.to_string());
    let mint_signature = if instructions.mint.instruction_set.instructions.is_empty() {
        None
    } else {
        let signers: Vec<&Keypair> = instructions
            .mint
            .instruction_set
            .signers
            .iter()
            .collect();
        Some(submit_transaction(
            &rpc,
            payer,
            &signers,
            instructions.mint.instruction_set.instructions,
        )?)
    };

    let lp_pool_signature = if instructions.lp_pool.instruction_set.instructions.is_empty() {
        None
    } else {
        let signers: Vec<&Keypair> = instructions
            .lp_pool
            .instruction_set
            .signers
            .iter()
            .collect();
        Some(submit_transaction(
            &rpc,
            payer,
            &signers,
            instructions.lp_pool.instruction_set.instructions,
        )?)
    };

    let mut vault_results = Vec::with_capacity(instructions.vaults.len());
    for vault in instructions.vaults {
        let signature = if vault.instruction_set.instructions.is_empty() {
            None
        } else {
            let signers: Vec<&Keypair> = vault.instruction_set.signers.iter().collect();
            Some(submit_transaction(
                &rpc,
                payer,
                &signers,
                vault.instruction_set.instructions,
            )?)
        };
        vault_results.push(VaultResult {
            label: vault.label,
            address: vault.address,
            beneficiary: vault.beneficiary,
            schedule: vault.schedule,
            signature,
        });
    }

    Ok(LaunchResult {
        name: instructions.name,
        program_ids: instructions.program_ids,
        mint: MintResult {
            address: instructions.mint.address,
            symbol: instructions.mint.symbol,
            decimals: instructions.mint.decimals,
            authority: instructions.mint.authority,
            signature: mint_signature,
        },
        lp_pool: LpPoolResult {
            address: instructions.lp_pool.address,
            base_mint: instructions.lp_pool.base_mint,
            quote_mint: instructions.lp_pool.quote_mint,
            signature: lp_pool_signature,
        },
        vaults: vault_results,
    })
}

pub fn execute_launch(
    rpc_url: &str,
    payer: &Keypair,
    plan: LaunchPlan,
) -> Result<LaunchResult, BackendError> {
    let instructions = build_launch_instructions(plan)?;
    submit_launch_transactions(rpc_url, payer, instructions)
}

fn submit_transaction(
    rpc: &RpcClient,
    payer: &Keypair,
    additional_signers: &[&Keypair],
    instructions: Vec<Instruction>,
) -> Result<Signature, BackendError> {
    let recent_blockhash = rpc
        .get_latest_blockhash()
        .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))?;
    let mut signers: Vec<&dyn Signer> = Vec::with_capacity(additional_signers.len() + 1);
    signers.push(payer);
    for signer in additional_signers {
        signers.push(*signer);
    }
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &signers,
        recent_blockhash,
    );
    rpc.send_and_confirm_transaction(&transaction)
        .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))
}
