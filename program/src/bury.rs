use ore_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Bury ORE and distribute yield to stakers.
pub fn process_bury(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = Bury::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, sender_info, board_info, config_info, mint_info, treasury_info, treasury_ore_info, token_program, ore_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info.as_account::<Config>(&ore_api::ID)?;
    config_info.has_seeds(&[CONFIG, &config.mint.to_bytes()], &ore_api::ID)?;
    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(&signer_info.key, &config.mint)?;
    board_info.as_account_mut::<Board>(&ore_api::ID)?;
    board_info.has_seeds(&[BOARD, &config.mint.to_bytes()], &ore_api::ID)?;
    mint_info.has_address(&config.mint)?.as_mint()?;
    let treasury = treasury_info.as_account_mut::<Treasury>(&ore_api::ID)?;
    treasury_info.has_seeds(&[TREASURY, &config.mint.to_bytes()], &ore_api::ID)?;
    treasury_ore_info.as_associated_token_account(treasury_info.key, &config.mint)?;
    token_program.is_program(&spl_token::ID)?;
    ore_program.is_program(&ore_api::ID)?;

    // Transfer ORE from sender to treasury.
    let amount = sender.amount().min(amount);
    transfer(
        signer_info,
        sender_info,
        treasury_ore_info,
        token_program,
        amount,
    )?;

    // Share some ORE with stakers.
    let mut shared_amount = 0;
    if treasury.total_staked > 0 {
        shared_amount = amount.saturating_mul(config.stake_bps) / DENOMINATOR_BPS;
        treasury.stake_rewards_factor +=
            Numeric::from_fraction(shared_amount, treasury.total_staked);
    }
    sol_log(&format!(
        "💰 Shared {} ORE",
        amount_to_ui_amount(shared_amount, TOKEN_DECIMALS)
    ));

    // Burn ORE.
    let burn_amount = amount - shared_amount;
    burn_signed(
        treasury_ore_info,
        mint_info,
        treasury_info,
        token_program,
        burn_amount,
        &[TREASURY, &config.mint.to_bytes()],
    )?;

    sol_log(
        &format!(
            "🔥 Buried {} ORE",
            amount_to_ui_amount(burn_amount, TOKEN_DECIMALS)
        )
        .as_str(),
    );

    // Emit event.
    let mint = mint_info.as_mint()?;
    program_log(
        config.mint,
        &[board_info.clone(), ore_program.clone()],
        BuryEvent {
            disc: 1,
            ore_buried: burn_amount,
            ore_shared: shared_amount,
            sol_amount: 0,
            new_circulating_supply: mint.supply(),
            ts: Clock::get()?.unix_timestamp,
        }
        .to_bytes(),
    )?;

    Ok(())
}
