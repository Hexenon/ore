use std::path::{Path, PathBuf};

use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::keypair::Keypair;

use crate::error::BackendError;

pub fn load_keypair(path: impl AsRef<Path>) -> Result<Keypair, BackendError> {
    let path = path.as_ref();
    ensure_secure_permissions(path)?;
    read_keypair_file(path).map_err(|source| BackendError::WalletParse {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(unix)]
fn ensure_secure_permissions(path: &Path) -> Result<(), BackendError> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = std::fs::metadata(path).map_err(|source| BackendError::WalletRead {
        path: path.to_path_buf(),
        source,
    })?;
    let mode = metadata.permissions().mode();
    if mode & 0o077 != 0 {
        return Err(BackendError::InsecureWalletPermissions {
            path: path.to_path_buf(),
        });
    }
    Ok(())
}

#[cfg(not(unix))]
fn ensure_secure_permissions(_path: &Path) -> Result<(), BackendError> {
    Ok(())
}

pub fn wallet_path_display(path: &Path) -> String {
    path.display().to_string()
}

pub fn normalize_wallet_path(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().to_path_buf()
}
