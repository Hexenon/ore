use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::hash::hashv;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;

use crate::instruction::RewardsLockInstruction;
use crate::{VaultSchedule, VaultState};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = RewardsLockInstruction::try_from_slice(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    match instruction {
        RewardsLockInstruction::InitializeVault {
            beneficiary,
            schedule,
        } => process_initialize_vault(program_id, accounts, beneficiary, schedule),
    }
}

fn process_initialize_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    beneficiary: Pubkey,
    schedule: VaultSchedule,
) -> ProgramResult {
    let mut account_iter = accounts.iter();
    let vault_account = next_account_info(&mut account_iter)?;
    let payer_account = next_account_info(&mut account_iter)?;
    let beneficiary_account = next_account_info(&mut account_iter)?;
    let system_program_account = next_account_info(&mut account_iter)?;

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if system_program_account.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    if beneficiary_account.key != &beneficiary {
        return Err(ProgramError::InvalidArgument);
    }
    if !vault_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let schedule_hash = vault_schedule_hash(&schedule);
    let (expected_vault_address, bump) = Pubkey::find_program_address(
        &[b"vault", beneficiary.as_ref(), schedule_hash.as_ref()],
        program_id,
    );
    if expected_vault_address != *vault_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let vault_state = VaultState {
        beneficiary,
        schedule,
        bump,
    };
    let vault_data = borsh::to_vec(&vault_state).map_err(|_| ProgramError::InvalidAccountData)?;
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(vault_data.len());

    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            vault_account.key,
            lamports,
            vault_data.len() as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            vault_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            b"vault",
            beneficiary.as_ref(),
            schedule_hash.as_ref(),
            &[bump],
        ]],
    )?;

    vault_account.data.borrow_mut()[..vault_data.len()].copy_from_slice(&vault_data);
    Ok(())
}

fn vault_schedule_hash(schedule: &VaultSchedule) -> solana_program::hash::Hash {
    let cliff_flag: u8 = if schedule.cliff_ts.is_some() { 1 } else { 0 };
    let cliff_ts = schedule.cliff_ts.unwrap_or_default();
    hashv(&[
        b"vault_schedule",
        &schedule.start_ts.to_le_bytes(),
        &[cliff_flag],
        &cliff_ts.to_le_bytes(),
        &schedule.period_seconds.to_le_bytes(),
        &schedule.release_per_period.to_le_bytes(),
        &schedule.period_count.to_le_bytes(),
    ])
}
