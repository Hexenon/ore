use std::path::PathBuf;
use std::str::FromStr;

use rewards_lock::VaultSchedule;
use solana_sdk::pubkey::Pubkey;

use crate::config::{
    LaunchConfig, LpPoolConfig, MintConfig, OutputConfig, ProgramIdsConfig, VaultConfig,
    VaultScheduleConfig,
};

#[derive(Debug, serde::Serialize)]
struct LaunchOutput {
    name: Option<String>,
    program_ids: ProgramIdsOutput,
    mint: MintOutput,
    lp_pool: LpPoolOutput,
    vaults: Vec<VaultOutput>,
}

#[derive(Debug, serde::Serialize)]
struct ProgramIdsOutput {
    ore: String,
    mining: String,
    rewards_lock: String,
}

#[derive(Debug, serde::Serialize)]
struct MintOutput {
    address: String,
    symbol: String,
    decimals: u8,
    authority: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct LpPoolOutput {
    address: String,
    base_mint: String,
    quote_mint: String,
}

#[derive(Debug, serde::Serialize)]
struct VaultOutput {
    label: Option<String>,
    address: String,
    beneficiary: String,
    schedule: VaultScheduleConfig,
}

pub fn run(config_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = LaunchConfig::load_from_path(&config_path)?;
    let output = build_output(&config)?;

    print_summary(&output);

    if let Some(output_config) = config.output {
        write_output(&output_config, &output)?;
    }

    Ok(())
}

fn build_output(config: &LaunchConfig) -> Result<LaunchOutput, Box<dyn std::error::Error>> {
    let program_ids = resolve_program_ids(&config.programs)?;
    let mint = resolve_mint(&config.mint)?;
    let lp_pool = resolve_lp_pool(&config.lp_pool, &mint)?;
    let vaults = resolve_vaults(&config.vaults)?;

    Ok(LaunchOutput {
        name: config.name.clone(),
        program_ids,
        mint,
        lp_pool,
        vaults,
    })
}

fn resolve_program_ids(
    programs: &ProgramIdsConfig,
) -> Result<ProgramIdsOutput, Box<dyn std::error::Error>> {
    Ok(ProgramIdsOutput {
        ore: resolve_pubkey("programs.ore", programs.ore.as_deref())?.to_string(),
        mining: resolve_pubkey("programs.mining", programs.mining.as_deref())?.to_string(),
        rewards_lock: resolve_pubkey("programs.rewards_lock", programs.rewards_lock.as_deref())?
            .to_string(),
    })
}

fn resolve_mint(mint: &MintConfig) -> Result<MintOutput, Box<dyn std::error::Error>> {
    let address = resolve_pubkey("mint.address", mint.address.as_deref())?.to_string();
    let authority = match &mint.authority {
        Some(value) => Some(parse_pubkey("mint.authority", value)?.to_string()),
        None => None,
    };
    Ok(MintOutput {
        address,
        symbol: mint.symbol.clone(),
        decimals: mint.decimals,
        authority,
    })
}

fn resolve_lp_pool(
    lp_pool: &LpPoolConfig,
    mint: &MintOutput,
) -> Result<LpPoolOutput, Box<dyn std::error::Error>> {
    let address = resolve_pubkey("lp_pool.address", lp_pool.address.as_deref())?.to_string();
    let base_mint = match &lp_pool.base_mint {
        Some(value) => parse_pubkey("lp_pool.base_mint", value)?.to_string(),
        None => mint.address.clone(),
    };
    let quote_mint = parse_pubkey("lp_pool.quote_mint", &lp_pool.quote_mint)?.to_string();
    Ok(LpPoolOutput {
        address,
        base_mint,
        quote_mint,
    })
}

fn resolve_vaults(vaults: &[VaultConfig]) -> Result<Vec<VaultOutput>, Box<dyn std::error::Error>> {
    vaults
        .iter()
        .map(|vault| {
            let address = resolve_pubkey("vaults.address", vault.address.as_deref())?.to_string();
            let _schedule = to_schedule(&vault.schedule)?;
            Ok(VaultOutput {
                label: vault.label.clone(),
                address,
                beneficiary: vault.beneficiary.clone(),
                schedule: vault.schedule.clone(),
            })
        })
        .collect()
}

fn to_schedule(
    schedule: &VaultScheduleConfig,
) -> Result<VaultSchedule, Box<dyn std::error::Error>> {
    if schedule.period_seconds <= 0 {
        return Err("vault schedule period_seconds must be positive".into());
    }
    if schedule.period_count == 0 {
        return Err("vault schedule period_count must be greater than zero".into());
    }
    Ok(VaultSchedule {
        start_ts: schedule.start_ts,
        cliff_ts: schedule.cliff_ts,
        period_seconds: schedule.period_seconds,
        release_per_period: schedule.release_per_period,
        period_count: schedule.period_count,
    })
}

fn resolve_pubkey(label: &str, value: Option<&str>) -> Result<Pubkey, Box<dyn std::error::Error>> {
    match value {
        Some(value) => parse_pubkey(label, value),
        None => Ok(Pubkey::new_unique()),
    }
}

fn parse_pubkey(label: &str, value: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    Pubkey::from_str(value).map_err(|err| format!("invalid {label} pubkey: {err}").into())
}

fn print_summary(output: &LaunchOutput) {
    if let Some(name) = &output.name {
        println!("Launch: {name}");
    }
    println!("Program IDs:");
    println!("  ORE: {}", output.program_ids.ore);
    println!("  Mining: {}", output.program_ids.mining);
    println!("  Rewards lock: {}", output.program_ids.rewards_lock);
    println!();
    println!("Mint:");
    println!("  Address: {}", output.mint.address);
    println!(
        "  Symbol: {} (decimals {})",
        output.mint.symbol, output.mint.decimals
    );
    if let Some(authority) = &output.mint.authority {
        println!("  Authority: {authority}");
    }
    println!();
    println!("LP Pool:");
    println!("  Address: {}", output.lp_pool.address);
    println!("  Base mint: {}", output.lp_pool.base_mint);
    println!("  Quote mint: {}", output.lp_pool.quote_mint);
    if output.vaults.is_empty() {
        println!("\nVaults: none");
    } else {
        println!("\nVaults:");
        for (index, vault) in output.vaults.iter().enumerate() {
            let label = vault
                .label
                .as_ref()
                .map(|value| value.as_str())
                .unwrap_or("unnamed");
            println!("  {}. {} -> {}", index + 1, label, vault.address);
            println!("     Beneficiary: {}", vault.beneficiary);
            println!(
                "     Schedule: start={} period={}s count={} release_per_period={}",
                vault.schedule.start_ts,
                vault.schedule.period_seconds,
                vault.schedule.period_count,
                vault.schedule.release_per_period
            );
        }
    }
}

fn write_output(
    output_config: &OutputConfig,
    output: &LaunchOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string_pretty(output)?;
    std::fs::write(&output_config.path, serialized)?;
    println!("\nWrote launch summary to {}", output_config.path.display());
    Ok(())
}
