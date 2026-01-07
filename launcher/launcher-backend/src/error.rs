use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("failed to read config file {path}: {source}")]
    ConfigRead {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse config: {0}")]
    ConfigParse(String),
    #[error("wallet path is not configured")]
    MissingWalletPath,
    #[error("wallet file {path} has insecure permissions")]
    InsecureWalletPermissions { path: PathBuf },
    #[error("failed to read wallet file {path}: {source}")]
    WalletRead {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("invalid wallet file {path}: {source}")]
    WalletParse {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("signing failed: {0}")]
    SigningFailed(String),
    #[error("on-chain query failed: {0}")]
    OnChainQueryFailed(String),
    #[error("action execution failed: {0}")]
    ActionExecutionFailed(String),
}
