use std::path::PathBuf;
use std::str::FromStr;

use launcher_backend::{
    execute_launch, LaunchPlan, LaunchResult, LpPoolPlan, MintPlan, ProgramIdsPlan, VaultPlan,
};
use launcher_backend::wallet::load_keypair;
use rewards_lock::VaultSchedule;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

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
    transactions: LaunchTransactionsOutput,
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

#[derive(Debug, serde::Serialize)]
struct LaunchTransactionsOutput {
    mint: Option<String>,
    lp_pool: Option<String>,
    vaults: Vec<VaultTransactionOutput>,
}

#[derive(Debug, serde::Serialize)]
struct VaultTransactionOutput {
    label: Option<String>,
    address: String,
    signature: Option<String>,
}

pub fn run(config_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = LaunchConfig::load_from_path(&config_path)?;
    let plan = build_plan(&config)?;
    let payer = load_keypair(&config.payer_wallet)?;
    let result = execute_launch(&config.rpc_url, &payer, plan)?;
    let output = build_output(&result);

    print_summary(&output);

    if let Some(output_config) = config.output {
        write_output(&output_config, &output)?;
    }

    Ok(())
}

fn build_plan(config: &LaunchConfig) -> Result<LaunchPlan, Box<dyn std::error::Error>> {
    let program_ids = resolve_program_ids(&config.programs)?;
    let mint = resolve_mint(&config.mint)?;
    let lp_pool = resolve_lp_pool(&config.lp_pool, mint.address)?;
    let vaults = resolve_vaults(&config.vaults)?;

    Ok(LaunchPlan {
        name: config.name.clone(),
        program_ids,
        mint,
        lp_pool,
        vaults,
    })
}

fn build_output(result: &LaunchResult) -> LaunchOutput {
    LaunchOutput {
        name: result.name.clone(),
        program_ids: ProgramIdsOutput {
            ore: result.program_ids.ore.to_string(),
            mining: result.program_ids.mining.to_string(),
            rewards_lock: result.program_ids.rewards_lock.to_string(),
        },
        mint: MintOutput {
            address: result.mint.address.to_string(),
            symbol: result.mint.symbol.clone(),
            decimals: result.mint.decimals,
            authority: result.mint.authority.as_ref().map(ToString::to_string),
        },
        lp_pool: LpPoolOutput {
            address: result.lp_pool.address.to_string(),
            base_mint: result.lp_pool.base_mint.to_string(),
            quote_mint: result.lp_pool.quote_mint.to_string(),
        },
        vaults: result
            .vaults
            .iter()
            .map(|vault| VaultOutput {
                label: vault.label.clone(),
                address: vault.address.to_string(),
                beneficiary: vault.beneficiary.to_string(),
                schedule: VaultScheduleConfig {
                    start_ts: vault.schedule.start_ts,
                    cliff_ts: vault.schedule.cliff_ts,
                    period_seconds: vault.schedule.period_seconds,
                    release_per_period: vault.schedule.release_per_period,
                    period_count: vault.schedule.period_count,
                },
            })
            .collect(),
        transactions: LaunchTransactionsOutput {
            mint: result.mint.signature.as_ref().map(ToString::to_string),
            lp_pool: result.lp_pool.signature.as_ref().map(ToString::to_string),
            vaults: result
                .vaults
                .iter()
                .map(|vault| VaultTransactionOutput {
                    label: vault.label.clone(),
                    address: vault.address.to_string(),
                    signature: vault.signature.as_ref().map(ToString::to_string),
                })
                .collect(),
        },
    }
}

fn resolve_program_ids(
    programs: &ProgramIdsConfig,
) -> Result<ProgramIdsPlan, Box<dyn std::error::Error>> {
    Ok(ProgramIdsPlan {
        ore: resolve_pubkey("programs.ore", programs.ore.as_deref())?,
        mining: resolve_pubkey("programs.mining", programs.mining.as_deref())?,
        rewards_lock: resolve_pubkey("programs.rewards_lock", programs.rewards_lock.as_deref())?,
    })
}

fn resolve_mint(mint: &MintConfig) -> Result<MintPlan, Box<dyn std::error::Error>> {
    let (address, keypair) = resolve_pubkey_with_keypair("mint.address", mint.address.as_deref())?;
    let authority = match &mint.authority {
        Some(value) => Some(parse_pubkey("mint.authority", value)?),
        None => None,
    };
    Ok(MintPlan {
        address,
        symbol: mint.symbol.clone(),
        decimals: mint.decimals,
        authority,
        keypair,
    })
}

fn resolve_lp_pool(
    lp_pool: &LpPoolConfig,
    mint_address: Pubkey,
) -> Result<LpPoolPlan, Box<dyn std::error::Error>> {
    let (address, keypair) =
        resolve_pubkey_with_keypair("lp_pool.address", lp_pool.address.as_deref())?;
    let base_mint = match &lp_pool.base_mint {
        Some(value) => parse_pubkey("lp_pool.base_mint", value)?,
        None => mint_address,
    };
    let quote_mint = parse_pubkey("lp_pool.quote_mint", &lp_pool.quote_mint)?;
    Ok(LpPoolPlan {
        address,
        base_mint,
        quote_mint,
        keypair,
    })
}

fn resolve_vaults(vaults: &[VaultConfig]) -> Result<Vec<VaultPlan>, Box<dyn std::error::Error>> {
    vaults
        .iter()
        .map(|vault| {
            let (address, keypair) =
                resolve_pubkey_with_keypair("vaults.address", vault.address.as_deref())?;
            let schedule = to_schedule(&vault.schedule)?;
            Ok(VaultPlan {
                label: vault.label.clone(),
                address,
                beneficiary: parse_pubkey("vaults.beneficiary", &vault.beneficiary)?,
                schedule,
                keypair,
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

fn parse_pubkey(label: &str, value: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    Pubkey::from_str(value).map_err(|err| format!("invalid {label} pubkey: {err}").into())
}

fn resolve_pubkey(label: &str, value: Option<&str>) -> Result<Pubkey, Box<dyn std::error::Error>> {
    match value {
        Some(value) => parse_pubkey(label, value),
        None => Ok(Pubkey::new_unique()),
    }
}

fn resolve_pubkey_with_keypair(
    label: &str,
    value: Option<&str>,
) -> Result<(Pubkey, Option<Keypair>), Box<dyn std::error::Error>> {
    match value {
        Some(value) => Ok((parse_pubkey(label, value)?, None)),
        None => {
            let keypair = Keypair::new();
            Ok((keypair.pubkey(), Some(keypair)))
        }
    }
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
    if let Some(signature) = &output.transactions.mint {
        println!("  Create signature: {signature}");
    }
    println!();
    println!("LP Pool:");
    println!("  Address: {}", output.lp_pool.address);
    println!("  Base mint: {}", output.lp_pool.base_mint);
    println!("  Quote mint: {}", output.lp_pool.quote_mint);
    if let Some(signature) = &output.transactions.lp_pool {
        println!("  Create signature: {signature}");
    }
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
            if let Some(signature) = output
                .transactions
                .vaults
                .iter()
                .find(|entry| entry.address == vault.address)
                .and_then(|entry| entry.signature.as_ref())
            {
                println!("     Create signature: {signature}");
            }
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
