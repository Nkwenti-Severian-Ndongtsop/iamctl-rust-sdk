use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::server::{RequestHandler, JsonRpcRequest, JsonRpcClient};
use iamctl_rust_sdk::provider::{Provider, ProviderMetadata, ProviderCapabilities, PlanRequest, PlanResponse, ApplyRequest, ApplyResponse};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::json;
use tempfile::tempdir;

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "test-provider".to_string(),
            version: "0.1.0".to_string(),
            author: Some("Test Author".to_string()),
            repository: Some("https://github.com/test/repo".to_string()),
            description: Some("A test provider for integration testing".to_string()),
        }
    }
    
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supported_resources: vec!["test".to_string()],
            can_import: false,
            can_validate: true,
        }
    }

    async fn plan(&self, _request: PlanRequest) -> anyhow::Result<PlanResponse> {
        Ok(PlanResponse {
            changes: vec![],
        })
    }

    async fn apply(&self, _request: ApplyRequest) -> anyhow::Result<ApplyResponse> {
        Ok(ApplyResponse {
            successful_addresses: vec![],
            failed_addresses: vec![],
        })
    }
}

#[tokio::test]
async fn test_request_handler_metadata() {
    let provider = Arc::new(MockProvider);
    let handler = RequestHandler::new(provider);
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "metadata".to_string(),
        params: json!({}),
        id: Some(json!(1)),
    };
    
    let response = handler.handle(request).await;
    assert!(response.result.is_some());
    let result = response.result.unwrap();
    assert_eq!(result["name"], "test-provider");
}

#[tokio::test]
async fn test_request_handler_validate() {
    let provider = Arc::new(MockProvider);
    let handler = RequestHandler::new(provider);
    
    let mut spec = HashMap::new();
    spec.insert("name".to_string(), json!("test"));
    
    let resource = Resource {
        address: ResourceAddress {
            resource_type: "test".to_string(),
            name: "test-resource".to_string(),
            namespace: None,
        },
        spec,
        metadata: HashMap::new(),
    };
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "validate".to_string(),
        params: json!({ "resources": [resource] }),
        id: Some(json!(1)),
    };
    
    let response = handler.handle(request).await;
    assert!(response.result.is_some());
    assert_eq!(response.result.unwrap()["valid"], true);
}

#[tokio::test]
async fn test_json_rpc_client_mock() {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("test.sock");
    let socket_str = socket_path.to_str().unwrap();
    
    let client = JsonRpcClient::launch(socket_str);
    assert!(client.is_err()); // Should fail since no process at path
}
