use crate::provider::{ApplyRequest, ImportRequest, PlanRequest, Provider, ValidateRequest};
use crate::server::types::{JsonRpcRequest, JsonRpcResponse};
use std::sync::Arc;

/// Handles the routing and processing of JSON-RPC requests to the provider.
pub struct RequestHandler<P: Provider> {
    provider: Arc<P>,
}

impl<P: Provider + 'static> RequestHandler<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    pub async fn handle(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(serde_json::Value::Null);

        match request.method.as_str() {
            "metadata" => self.handle_metadata(id).await,
            "capabilities" => self.handle_capabilities(id).await,
            "plan" => self.handle_plan(id, request.params).await,
            "apply" => self.handle_apply(id, request.params).await,
            "validate" => self.handle_validate(id, request.params).await,
            "import" => self.handle_import(id, request.params).await,
            _ => {
                JsonRpcResponse::error(id, -32601, format!("Method not found: {}", request.method))
            }
        }
    }

    async fn handle_metadata(&self, id: serde_json::Value) -> JsonRpcResponse {
        let metadata = self.provider.metadata();
        match serde_json::to_value(metadata) {
            Ok(val) => JsonRpcResponse::success(id, val),
            Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
        }
    }

    async fn handle_capabilities(&self, id: serde_json::Value) -> JsonRpcResponse {
        let capabilities = self.provider.capabilities();
        match serde_json::to_value(capabilities) {
            Ok(val) => JsonRpcResponse::success(id, val),
            Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
        }
    }

    async fn handle_plan(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: PlanRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
        };

        match self.provider.plan(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {}", e)),
        }
    }

    async fn handle_apply(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ApplyRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
        };

        match self.provider.apply(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {}", e)),
        }
    }

    async fn handle_validate(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ValidateRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
        };

        match self.provider.validate(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {}", e)),
        }
    }

    async fn handle_import(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ImportRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
        };

        match self.provider.import(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {}", e)),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {}", e)),
        }
    }
}
