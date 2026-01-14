use ore_api::prelude::*;
use solana_program::system_program;
use steel::*;

pub fn process_initialize_lp_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'_>],
    data: &[u8],
) -> ProgramResult {
    let args = InitializeLpPool::try_from_bytes(data)?;
    let base_mint = Pubkey::new_from_array(args.base_mint);
    let quote_mint = Pubkey::new_from_array(args.quote_mint);

    let [lp_pool_info, payer_info, base_mint_info, quote_mint_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_info.is_signer {
        return Err(OreError::MissingPayerSignature.into());
    }

    if *base_mint_info.key != base_mint || *quote_mint_info.key != quote_mint {
        return Err(OreError::MintAddressMismatch.into());
    }

    let (expected_lp_pool, bump) =
        Pubkey::find_program_address(&[LP_POOL, base_mint.as_ref()], program_id);
    if *lp_pool_info.key != expected_lp_pool {
        return Err(OreError::LpPoolPdaMismatch.into());
    }

    system_program_info.is_program(&system_program::ID)?;

    if !lp_pool_info.data_is_empty() {
        return Err(OreError::LpPoolAlreadyInitialized.into());
    }

    create_program_account::<LpPool>(
        lp_pool_info,
        system_program_info,
        payer_info,
        program_id,
        &[LP_POOL, base_mint.as_ref()],
    )?;

    let lp_pool = lp_pool_info.as_account_mut::<LpPool>(program_id)?;
    lp_pool.base_mint = base_mint;
    lp_pool.quote_mint = quote_mint;
    lp_pool.authority = *payer_info.key;
    lp_pool.bump = bump;

    Ok(())
}
