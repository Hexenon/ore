//! Backend orchestration for launcher services.

pub struct BackendStatus {
    pub healthy: bool,
}

impl BackendStatus {
    pub fn ok() -> Self {
        Self { healthy: true }
    }
}
