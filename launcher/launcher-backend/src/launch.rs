use rewards_lock::VaultSchedule;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

use crate::error::BackendError;

#[derive(Debug)]
pub struct LaunchPlan {
    pub name: Option<String>,
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

pub fn execute_launch(
    rpc_url: &str,
    payer: &Keypair,
    plan: LaunchPlan,
) -> Result<LaunchResult, BackendError> {
    let rpc = RpcClient::new(rpc_url.to_string());
    let resolved_mint_authority = plan.mint.authority.unwrap_or_else(|| payer.pubkey());
    let mint_signature = if let Some(mint_keypair) = plan.mint.keypair.as_ref() {
        let rent = rpc
            .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
            .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))?;
        let instructions = vec![
            system_instruction::create_account(
                &payer.pubkey(),
                &plan.mint.address,
                rent,
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
        Some(submit_transaction(&rpc, payer, &[mint_keypair], instructions)?)
    } else {
        None
    };

    let lp_pool_signature = if let Some(lp_keypair) = plan.lp_pool.keypair.as_ref() {
        let rent = rpc
            .get_minimum_balance_for_rent_exemption(0)
            .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))?;
        let instructions = vec![system_instruction::create_account(
            &payer.pubkey(),
            &plan.lp_pool.address,
            rent,
            0,
            &plan.program_ids.mining,
        )];
        Some(submit_transaction(&rpc, payer, &[lp_keypair], instructions)?)
    } else {
        None
    };

    let mut vault_results = Vec::with_capacity(plan.vaults.len());
    for vault in &plan.vaults {
        let signature = if let Some(vault_keypair) = vault.keypair.as_ref() {
            let rent = rpc
                .get_minimum_balance_for_rent_exemption(0)
                .map_err(|err| BackendError::ActionExecutionFailed(err.to_string()))?;
            let instructions = vec![system_instruction::create_account(
                &payer.pubkey(),
                &vault.address,
                rent,
                0,
                &plan.program_ids.rewards_lock,
            )];
            Some(submit_transaction(&rpc, payer, &[vault_keypair], instructions)?)
        } else {
            None
        };
        vault_results.push(VaultResult {
            label: vault.label.clone(),
            address: vault.address,
            beneficiary: vault.beneficiary,
            schedule: vault.schedule,
            signature,
        });
    }

    Ok(LaunchResult {
        name: plan.name,
        program_ids: plan.program_ids,
        mint: MintResult {
            address: plan.mint.address,
            symbol: plan.mint.symbol,
            decimals: plan.mint.decimals,
            authority: if plan.mint.keypair.is_some() {
                Some(resolved_mint_authority)
            } else {
                plan.mint.authority
            },
            signature: mint_signature,
        },
        lp_pool: LpPoolResult {
            address: plan.lp_pool.address,
            base_mint: plan.lp_pool.base_mint,
            quote_mint: plan.lp_pool.quote_mint,
            signature: lp_pool_signature,
        },
        vaults: vault_results,
    })
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
