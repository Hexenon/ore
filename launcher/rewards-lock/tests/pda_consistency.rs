use rewards_lock::pda::{lp_pool_pda, vault_pda, vault_schedule_hash, LP_POOL_SEED, VAULT_SEED};
use rewards_lock::{pda::VAULT_SCHEDULE_SEED, VaultSchedule};
use solana_program::pubkey::Pubkey as ProgramPubkey;
use solana_sdk::hash::hashv as sdk_hashv;
use solana_sdk::pubkey::Pubkey as SdkPubkey;

#[test]
fn lp_pool_pda_matches_sdk_derivation() {
    let mint = ProgramPubkey::new_unique();
    let program_id = ProgramPubkey::new_unique();

    let (on_chain, _bump) = lp_pool_pda(mint, program_id);

    let mint_sdk = SdkPubkey::new_from_array(mint.to_bytes());
    let program_sdk = SdkPubkey::new_from_array(program_id.to_bytes());
    let (sdk, _sdk_bump) =
        SdkPubkey::find_program_address(&[LP_POOL_SEED, mint_sdk.as_ref()], &program_sdk);

    assert_eq!(on_chain.to_bytes(), sdk.to_bytes());
}

#[test]
fn vault_pda_matches_sdk_derivation() {
    let beneficiary = ProgramPubkey::new_unique();
    let program_id = ProgramPubkey::new_unique();
    let schedule = VaultSchedule {
        start_ts: 1_725_000_000,
        cliff_ts: Some(1_725_100_000),
        period_seconds: 86_400,
        release_per_period: 10_000,
        period_count: 180,
    };

    let (on_chain, _bump) = vault_pda(beneficiary, &schedule, program_id);
    let schedule_hash = vault_schedule_hash(&schedule);

    let beneficiary_sdk = SdkPubkey::new_from_array(beneficiary.to_bytes());
    let program_sdk = SdkPubkey::new_from_array(program_id.to_bytes());
    let cliff_flag: u8 = if schedule.cliff_ts.is_some() { 1 } else { 0 };
    let cliff_ts = schedule.cliff_ts.unwrap_or_default();
    let sdk_schedule_hash = sdk_hashv(&[
        VAULT_SCHEDULE_SEED,
        &schedule.start_ts.to_le_bytes(),
        &[cliff_flag],
        &cliff_ts.to_le_bytes(),
        &schedule.period_seconds.to_le_bytes(),
        &schedule.release_per_period.to_le_bytes(),
        &schedule.period_count.to_le_bytes(),
    ]);
    let (sdk, _sdk_bump) = SdkPubkey::find_program_address(
        &[
            VAULT_SEED,
            beneficiary_sdk.as_ref(),
            sdk_schedule_hash.as_ref(),
        ],
        &program_sdk,
    );

    assert_eq!(schedule_hash.to_bytes(), sdk_schedule_hash.to_bytes());
    assert_eq!(on_chain.to_bytes(), sdk.to_bytes());
}
