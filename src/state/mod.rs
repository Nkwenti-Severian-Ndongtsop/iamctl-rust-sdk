use crate::types::Resource;
use crate::utils::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// Represents the state of resources at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// Version of the state schema.
    pub version: u32,
    /// Map of resource addresses to resource data.
    pub resources: HashMap<String, Resource>,
    /// Optional metadata about the state.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            version: 1,
            resources: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Interface for state management backends.
#[async_trait]
pub trait StateBackend: Send + Sync {
    /// Load the state from the backend.
    async fn load(&self) -> Result<State>;

    /// Save the state to the backend.
    async fn save(&self, state: &State) -> Result<()>;
}

/// Interface for state locking to prevent concurrent modifications.
#[async_trait]
pub trait StateLocking: Send + Sync {
    /// Acquire a lock on the state.
    async fn lock(&self) -> Result<()>;

    /// Release the lock on the state.
    async fn unlock(&self) -> Result<()>;
}

/// A state backend that stores state in a local file.
pub struct FileBackend {
    path: PathBuf,
    lock_path: PathBuf,
}

impl FileBackend {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let path = path.into();
        let mut lock_path = path.clone();
        let _ = lock_path.set_extension("lock");
        Self { path, lock_path }
    }
}

#[async_trait]
impl StateLocking for FileBackend {
    async fn lock(&self) -> Result<()> {
        let mut attempts = 0;
        while attempts < 10 {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.lock_path)
                .await
            {
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    attempts += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Err(e) => {
                    return Err(Error::Internal(format!(
                        "Failed to create lock file: {e}"
                    )))
                }
            }
        }
        Err(Error::Internal(
            "Timeout waiting for state lock".to_string(),
        ))
    }

    async fn unlock(&self) -> Result<()> {
        if self.lock_path.exists() {
            fs::remove_file(&self.lock_path)
                .await
                .map_err(|e| Error::Internal(format!("Failed to remove lock file: {e}")))?;
        }
        Ok(())
    }
}

#[async_trait]
impl StateBackend for FileBackend {
    async fn load(&self) -> Result<State> {
        if !self.path.exists() {
            return Ok(State::default());
        }

        let metadata = fs::metadata(&self.path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get state file metadata: {e}")))?;

        if metadata.len() == 0 {
            return Ok(State::default());
        }

        let content = fs::read_to_string(&self.path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to read state file: {e}")))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::Internal(format!("Failed to parse state file: {e}")))
    }

    async fn save(&self, state: &State) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    Error::Internal(format!("Failed to create state directory: {e}"))
                })?;
            }
        }

        let content = serde_json::to_string_pretty(state)
            .map_err(|e| Error::Internal(format!("Failed to serialize state: {e}")))?;

        #[cfg(unix)]
        {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;

            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&self.path)
                .map_err(|e| {
                    Error::Internal(format!(
                        "Failed to open state file with restricted permissions: {e}"
                    ))
                })?;

            file.write_all(content.as_bytes())
                .map_err(|e| Error::Internal(format!("Failed to write state file: {e}")))?;
            file.flush()
                .map_err(|e| Error::Internal(format!("Failed to flush state file: {e}")))?;
        }

        #[cfg(not(unix))]
        {
            fs::write(&self.path, content)
                .await
                .map_err(|e| Error::Internal(format!("Failed to write state file: {}", e)))?;
        }

        Ok(())
    }
}
