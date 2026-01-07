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
    #[error("failed to read launch store {path}: {source}")]
    StoreRead { path: PathBuf, source: std::io::Error },
    #[error("failed to parse launch store {path}: {source}")]
    StoreParse { path: PathBuf, source: serde_json::Error },
    #[error("failed to serialize launch store: {source}")]
    StoreSerialize { source: serde_json::Error },
    #[error("failed to write launch store {path}: {source}")]
    StoreWrite { path: PathBuf, source: std::io::Error },
    #[error("launch {launch_id} for user {user_id} not found")]
    LaunchNotFound { user_id: String, launch_id: String },
}
