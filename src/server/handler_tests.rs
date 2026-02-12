#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::server::{JsonRpcRequest, JsonRpcResponse, RequestHandler};
    use std::sync::Arc;
    use serde_json::json;
    use async_trait::async_trait;

    struct TestProvider;

    #[async_trait]
    impl Provider for TestProvider {
        fn metadata(&self) -> ProviderMetadata {
            ProviderMetadata {
                name: "test-provider".to_string(),
                version: "1.0.0".to_string(),
                author: None,
                repository: None,
                description: None,
            }
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                supported_resources: vec!["realm".to_string()],
                can_import: false,
                can_validate: false,
            }
        }

        async fn plan(&self, _request: PlanRequest) -> anyhow::Result<PlanResponse> {
            Ok(PlanResponse { changes: vec![] })
        }

        async fn apply(&self, _request: ApplyRequest) -> anyhow::Result<ApplyResponse> {
            Ok(ApplyResponse {
                successful_addresses: vec![],
                failed_addresses: vec![],
            })
        }
    }

    #[tokio::test]
    async fn test_handle_metadata() {
        let provider = Arc::new(TestProvider);
        let handler = RequestHandler::new(provider);
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "metadata".to_string(),
            params: json!({}),
            id: Some(json!(1)),
        };

        let response = handler.handle(request).await;
        assert_eq!(response.id, json!(1));
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["name"], "test-provider");
    }

    #[tokio::test]
    async fn test_method_not_found() {
        let provider = Arc::new(TestProvider);
        let handler = RequestHandler::new(provider);
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "non_existent".to_string(),
            params: json!({}),
            id: Some(json!(1)),
        };

        let response = handler.handle(request).await;
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_none());
        assert_eq!(response.error.unwrap().code, -32601);
    }
}
