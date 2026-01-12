use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchConfig {
    pub name: Option<String>,
    pub rpc_url: String,
    pub payer_wallet: PathBuf,
    pub mint: MintConfig,
    pub lp_pool: LpPoolConfig,
    #[serde(default)]
    pub vaults: Vec<VaultConfig>,
    pub output: Option<OutputConfig>,
}

impl LaunchConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)?;
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(&contents)?,
            Some("toml") | _ => toml::from_str(&contents)?,
        };
        Ok(config)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramIdsConfig {
    pub ore: Option<String>,
    pub mining: Option<String>,
    pub rewards_lock: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub programs: ProgramIdsConfig,
}

impl LauncherConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)?;
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(&contents)?,
            Some("toml") | _ => toml::from_str(&contents)?,
        };
        Ok(config)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintConfig {
    /// Required client-generated address used for signing.
    pub address: Option<String>,
    pub symbol: String,
    #[serde(default = "default_decimals")]
    pub decimals: u8,
    pub authority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LpPoolConfig {
    pub base_mint: Option<String>,
    pub quote_mint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub label: Option<String>,
    pub beneficiary: String,
    pub schedule: VaultScheduleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultScheduleConfig {
    pub start_ts: i64,
    pub cliff_ts: Option<i64>,
    pub period_seconds: i64,
    pub release_per_period: u64,
    pub period_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub path: PathBuf,
}

fn default_decimals() -> u8 {
    11
}
