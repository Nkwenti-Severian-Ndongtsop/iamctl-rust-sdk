use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::server::{JsonRpcRequest, RequestHandler};
use iamctl_rust_sdk::provider::{ValidateRequest, ValidateResponse, ImportRequest, ImportResponse};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "test-provider".to_string(),
            version: "0.1.0".to_string(),
            author: None,
            repository: None,
            description: None,
        }
    }
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supported_resources: vec!["test".to_string()],
            can_import: true,
            can_validate: true,
        }
    }
    async fn plan(&self, _: PlanRequest) -> anyhow::Result<PlanResponse> {
        Ok(PlanResponse { changes: vec![] })
    }
    async fn apply(&self, _: ApplyRequest) -> anyhow::Result<ApplyResponse> {
        Ok(ApplyResponse {
            successful_addresses: vec![],
            failed_addresses: vec![],
        })
    }
    async fn validate(&self, _: ValidateRequest) -> anyhow::Result<ValidateResponse> {
        Ok(ValidateResponse {
            valid: true,
            errors: vec![],
        })
    }
    async fn import(&self, _: ImportRequest) -> anyhow::Result<ImportResponse> {
        Ok(ImportResponse { 
            resource: Resource {
                address: ResourceAddress {
                    resource_type: "test".to_string(),
                    name: "imported".to_string(),
                    namespace: None,
                },
                spec: std::collections::HashMap::new(),
                metadata: std::collections::HashMap::new(),
            }
        })
    }
}

#[tokio::test]
async fn test_handler_all_methods() {
    let handler = RequestHandler::new(Arc::new(MockProvider));

    // Test capabilities
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "capabilities".to_string(),
        params: json!({}),
        id: Some(json!(1)),
    }).await;
    assert!(resp.result.is_some());

    // Test plan
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "plan".to_string(),
        params: json!({ 
            "workspace_path": ".",
            "desired_state": [],
            "current_state": []
        }),
        id: Some(json!(2)),
    }).await;
    assert!(resp.result.is_some());

    // Test apply
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "apply".to_string(),
        params: json!({ "changes": [] }),
        id: Some(json!(3)),
    }).await;
    assert!(resp.result.is_some());

    // Test import
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "import".to_string(),
        params: json!({ 
            "address": {
                "resource_type": "test",
                "name": "imported",
                "namespace": null
            },
            "id": "some-id"
        }),
        id: Some(json!(4)),
    }).await;
    assert!(resp.result.is_some());

    // Test invalid method
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "unknown".to_string(),
        params: json!({}),
        id: Some(json!(5)),
    }).await;
    assert!(resp.error.is_some());
    assert_eq!(resp.error.unwrap().code, -32601);
}

#[tokio::test]
async fn test_handler_invalid_params() {
    let handler = RequestHandler::new(Arc::new(MockProvider));

    // Test plan with invalid params (missing workspace_path)
    let resp = handler.handle(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "plan".to_string(),
        params: json!({ "wrong": 123 }),
        id: Some(json!(1)),
    }).await;
    assert!(resp.error.is_some());
    assert_eq!(resp.error.unwrap().code, -32602);
}
