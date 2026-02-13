use iamctl_rust_sdk::state::{FileBackend, StateBackend};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::tempdir;

#[tokio::test]
async fn test_file_backend_permissions() {
    let dir = tempdir().unwrap();
    let state_path = dir.path().join("state.json");
    let backend = FileBackend::new(state_path.clone());

    // Trigger file creation
    backend.save(&Default::default()).await.unwrap();

    let metadata = fs::metadata(&state_path).unwrap();
    let mode = metadata.permissions().mode();

    // Check if only owner can read/write (0600)
    // Note: This test might fail on filesystems that don't support unix permissions
    // or if the default umask is very restrictive.
    assert_eq!(
        mode & 0o777,
        0o600,
        "State file should have 0600 permissions"
    );
}
