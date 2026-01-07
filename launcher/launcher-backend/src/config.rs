use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::BackendError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub wallet: WalletConfig,
    #[serde(default)]
    pub schedule: ScheduleConfig,
    #[serde(default)]
    pub policy: ActionPolicyConfig,
}

impl BackendConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, BackendError> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)
            .map_err(|source| BackendError::ConfigRead {
                path: path.to_path_buf(),
                source,
            })?;
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(&contents)
                .map_err(|err| BackendError::ConfigParse(err.to_string())),
            Some("toml") | _ => toml::from_str(&contents)
                .map_err(|err| BackendError::ConfigParse(err.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub path: Option<PathBuf>,
    pub env: Option<String>,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            path: None,
            env: Some("LAUNCHER_WALLET".to_string()),
        }
    }
}

impl WalletConfig {
    pub fn resolve_wallet_path(&self) -> Result<PathBuf, BackendError> {
        if let Some(env_name) = &self.env {
            if let Ok(env_value) = std::env::var(env_name) {
                if !env_value.trim().is_empty() {
                    return Ok(PathBuf::from(env_value));
                }
            }
        }
        self.path.clone().ok_or(BackendError::MissingWalletPath)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub poll_interval_secs: u64,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPolicyConfig {
    pub enable_reset: bool,
    pub enable_buyback: bool,
    pub reset_min_slot_gap: Option<u64>,
    pub buyback_min_treasury_lamports: Option<u64>,
}

impl Default for ActionPolicyConfig {
    fn default() -> Self {
        Self {
            enable_reset: true,
            enable_buyback: true,
            reset_min_slot_gap: None,
            buyback_min_treasury_lamports: None,
        }
    }
}
