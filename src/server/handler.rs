use crate::provider::{ApplyRequest, ImportRequest, PlanRequest, Provider, ValidateRequest};
use crate::server::types::{JsonRpcRequest, JsonRpcResponse};
use crate::validation::JsonSchemaValidator;
use crate::validation::SchemaValidator;
use crate::validation::ValidationResult;
use std::sync::Arc;

/// Handles the routing and processing of JSON-RPC requests to the provider.
pub struct RequestHandler<P: Provider> {
    provider: Arc<P>,
    validator: JsonSchemaValidator,
}

impl<P: Provider + 'static> RequestHandler<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self {
            provider,
            validator: JsonSchemaValidator::new(),
        }
    }

    fn format_validation_error(result: ValidationResult) -> String {
        if result.valid {
            return "Validation succeeded".to_string();
        }

        let mut msg = String::from("Schema validation failed");
        if !result.errors.is_empty() {
            msg.push_str(": ");
            for (idx, e) in result.errors.iter().enumerate() {
                if idx > 0 {
                    msg.push_str("; ");
                }
                msg.push_str(&format!("{}: {}", e.path, e.message));
            }
        }
        msg
    }

    fn validate_resource(
        &self,
        id: serde_json::Value,
        resource: &crate::types::Resource,
    ) -> Option<JsonRpcResponse> {
        match self.validator.validate(resource) {
            Ok(result) if result.valid => None,
            Ok(result) => {
                let is_schema_missing = result.errors.iter().all(|e| e.code == "SCHEMA_NOT_FOUND");

                if is_schema_missing {
                    None
                } else {
                    Some(JsonRpcResponse::error(
                        id,
                        -32602,
                        Self::format_validation_error(result),
                    ))
                }
            }
            Err(e) => Some(JsonRpcResponse::error(
                id,
                -32603,
                format!("Internal error: {e}"),
            )),
        }
    }

    /// Registers a JSON schema for a resource type by deriving it from a Rust type.
    pub fn register_type_schema<T: schemars::JsonSchema>(&mut self, resource_type: &str) {
        self.validator.add_type_schema::<T>(resource_type);
    }

    /// Registers a raw JSON schema for a resource type.
    pub fn register_schema(&mut self, resource_type: &str, schema: serde_json::Value) {
        self.validator.add_schema(resource_type, schema);
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
            Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
        }
    }

    async fn handle_capabilities(&self, id: serde_json::Value) -> JsonRpcResponse {
        let capabilities = self.provider.capabilities();
        match serde_json::to_value(capabilities) {
            Ok(val) => JsonRpcResponse::success(id, val),
            Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
        }
    }

    async fn handle_plan(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: PlanRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {e}")),
        };

        for resource in &request.desired_state {
            if let Some(resp) = self.validate_resource(id.clone(), resource) {
                return resp;
            }
        }

        match self.provider.plan(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {e}")),
        }
    }

    async fn handle_apply(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ApplyRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {e}")),
        };

        for change in &request.changes {
            if let Some(after) = &change.after {
                if let Some(resp) = self.validate_resource(id.clone(), after) {
                    return resp;
                }
            }
        }

        match self.provider.apply(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {e}")),
        }
    }

    async fn handle_validate(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ValidateRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {e}")),
        };

        for resource in &request.resources {
            if let Some(resp) = self.validate_resource(id.clone(), resource) {
                return resp;
            }
        }

        match self.provider.validate(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {e}")),
        }
    }

    async fn handle_import(
        &self,
        id: serde_json::Value,
        params: serde_json::Value,
    ) -> JsonRpcResponse {
        let request: ImportRequest = match serde_json::from_value(params) {
            Ok(req) => req,
            Err(e) => return JsonRpcResponse::error(id, -32602, format!("Invalid params: {e}")),
        };

        match self.provider.import(request).await {
            Ok(resp) => match serde_json::to_value(resp) {
                Ok(val) => JsonRpcResponse::success(id, val),
                Err(e) => JsonRpcResponse::error(id, -32603, format!("Internal error: {e}")),
            },
            Err(e) => JsonRpcResponse::error(id, -32000, format!("Provider error: {e}")),
        }
    }
}
