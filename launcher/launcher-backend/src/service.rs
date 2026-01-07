use std::fs;
use std::path::{Path, PathBuf};

use mining_api::{
    LaunchCreateRequest, LaunchCreateResponse, LaunchResultInfo, LaunchStatus, LaunchStatusRequest,
    LaunchStatusResponse,
};
use serde::{Deserialize, Serialize};

use crate::error::BackendError;

#[derive(Debug, Clone)]
pub struct LaunchService {
    store: LaunchStore,
}

impl LaunchService {
    pub fn new(store: LaunchStore) -> Self {
        Self { store }
    }

    pub fn create_launch(
        &mut self,
        request: LaunchCreateRequest,
    ) -> Result<LaunchCreateResponse, BackendError> {
        self.store.create_launch(request)
    }

    pub fn get_launch_status(
        &self,
        request: LaunchStatusRequest,
    ) -> Result<LaunchStatusResponse, BackendError> {
        self.store.get_launch_status(request)
    }

    pub fn update_launch_result(
        &mut self,
        user_id: impl Into<String>,
        launch_id: impl Into<String>,
        status: LaunchStatus,
        result: Option<LaunchResultInfo>,
    ) -> Result<LaunchStatusResponse, BackendError> {
        self.store
            .update_launch_result(user_id.into(), launch_id.into(), status, result)
    }
}

#[derive(Debug, Clone)]
pub struct LaunchStore {
    path: PathBuf,
    state: LaunchStoreState,
}

impl LaunchStore {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, BackendError> {
        let path = path.into();
        let state = if path.exists() {
            let contents = fs::read_to_string(&path)
                .map_err(|source| BackendError::StoreRead { path: path.clone(), source })?;
            serde_json::from_str(&contents).map_err(|source| BackendError::StoreParse {
                path: path.clone(),
                source,
            })?
        } else {
            LaunchStoreState::default()
        };
        Ok(Self { path, state })
    }

    pub fn create_launch(
        &mut self,
        request: LaunchCreateRequest,
    ) -> Result<LaunchCreateResponse, BackendError> {
        let launch_id = format!("launch-{}", self.state.next_id);
        self.state.next_id += 1;
        let record = LaunchRecord {
            launch_id: launch_id.clone(),
            user_id: request.user_id,
            status: LaunchStatus::Pending,
            config: request.config,
            result: None,
        };
        self.state.records.push(record);
        self.persist()?;
        Ok(LaunchCreateResponse {
            launch_id,
            status: LaunchStatus::Pending,
        })
    }

    pub fn get_launch_status(
        &self,
        request: LaunchStatusRequest,
    ) -> Result<LaunchStatusResponse, BackendError> {
        let record = self.find_record(&request.user_id, &request.launch_id)?;
        Ok(LaunchStatusResponse {
            launch_id: record.launch_id.clone(),
            status: record.status,
            result: record.result.clone(),
        })
    }

    pub fn update_launch_result(
        &mut self,
        user_id: String,
        launch_id: String,
        status: LaunchStatus,
        result: Option<LaunchResultInfo>,
    ) -> Result<LaunchStatusResponse, BackendError> {
        let record = self
            .find_record_mut(&user_id, &launch_id)
            .ok_or_else(|| BackendError::LaunchNotFound { user_id, launch_id })?;
        record.status = status;
        record.result = result.clone();
        self.persist()?;
        Ok(LaunchStatusResponse {
            launch_id: record.launch_id.clone(),
            status: record.status,
            result,
        })
    }

    fn persist(&self) -> Result<(), BackendError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|source| BackendError::StoreWrite { path: parent.to_path_buf(), source })?;
        }
        let serialized = serde_json::to_string_pretty(&self.state)
            .map_err(|source| BackendError::StoreSerialize { source })?;
        fs::write(&self.path, serialized)
            .map_err(|source| BackendError::StoreWrite { path: self.path.clone(), source })?;
        Ok(())
    }

    fn find_record(&self, user_id: &str, launch_id: &str) -> Result<&LaunchRecord, BackendError> {
        self.state
            .records
            .iter()
            .find(|record| record.user_id == user_id && record.launch_id == launch_id)
            .ok_or_else(|| BackendError::LaunchNotFound {
                user_id: user_id.to_string(),
                launch_id: launch_id.to_string(),
            })
    }

    fn find_record_mut(
        &mut self,
        user_id: &str,
        launch_id: &str,
    ) -> Option<&mut LaunchRecord> {
        self.state
            .records
            .iter_mut()
            .find(|record| record.user_id == user_id && record.launch_id == launch_id)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LaunchStoreState {
    next_id: u64,
    records: Vec<LaunchRecord>,
}

impl Default for LaunchStoreState {
    fn default() -> Self {
        Self {
            next_id: 1,
            records: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LaunchRecord {
    launch_id: String,
    user_id: String,
    status: LaunchStatus,
    config: serde_json::Value,
    result: Option<LaunchResultInfo>,
}
