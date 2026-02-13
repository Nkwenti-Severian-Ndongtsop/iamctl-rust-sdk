use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::state::{State, FileBackend, StateBackend, StateLocking};
use std::collections::HashMap;
use tempfile::NamedTempFile;
use serde_json::json;

#[tokio::test]
async fn test_file_backend_load_save() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();
    let backend = FileBackend::new(path);

    // 1. Test empty state
    let state = backend.load().await.unwrap();
    assert!(state.resources.is_empty());
    assert_eq!(state.version, 1);

    // 2. Save some state
    let mut resources = HashMap::new();
    let address = ResourceAddress {
        resource_type: "user".to_string(),
        name: "test-user".to_string(),
        namespace: None,
    };
    let resource = Resource {
        address: address.clone(),
        spec: HashMap::from([("email".to_string(), json!("test@example.com"))]),
        metadata: HashMap::new(),
    };
    resources.insert(address.to_string(), resource);

    let new_state = State {
        version: 1,
        resources,
        metadata: HashMap::from([("version".to_string(), "1".to_string())]),
    };

    backend.save(&new_state).await.unwrap();

    // 3. Load back and verify
    let loaded_state = backend.load().await.unwrap();
    assert_eq!(loaded_state.resources.len(), 1);
    assert_eq!(loaded_state.metadata.get("version").unwrap(), "1");
    assert!(loaded_state.resources.contains_key(&address.to_string()));
    assert_eq!(loaded_state.version, 1);
}

#[tokio::test]
async fn test_file_backend_locking() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();
    let backend = FileBackend::new(path);

    // 1. Acquire lock
    backend.lock().await.unwrap();

    // 2. Try to acquire again (should fail/timeout)
    let result = backend.lock().await;
    assert!(result.is_err());

    // 3. Unlock and try again (should succeed)
    backend.unlock().await.unwrap();
    backend.lock().await.unwrap();
    backend.unlock().await.unwrap();
}
