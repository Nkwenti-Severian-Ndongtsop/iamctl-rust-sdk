use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::server::JsonRpcServer;
use iamctl_rust_sdk::utils::init_logging;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserSpec {
    username: String,
    email: String,
    #[serde(default)]
    is_admin: bool,
}

struct ExampleProvider;

#[async_trait]
impl Provider for ExampleProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "example-basic-provider".to_string(),
            version: "0.1.0".to_string(),
            author: Some("IAMCTL Team".to_string()),
            repository: Some("https://github.com/iamctl/rust-sdk".to_string()),
            description: Some("A basic example provider demonstrating the IAMCTL Rust SDK".to_string()),
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supported_resources: vec!["user".to_string()],
            can_import: true,
            can_validate: true,
        }
    }

    async fn plan(&self, request: PlanRequest) -> anyhow::Result<PlanResponse> {
        let mut changes = vec![];
        for resource in request.resources {
            changes.push(Change {
                address: resource.address.clone(),
                change_type: ChangeType::Create,
                before: None,
                after: Some(resource),
                computed_fields: vec!["id".to_string()],
            });
        }
        Ok(PlanResponse { changes })
    }

    async fn apply(&self, request: ApplyRequest) -> anyhow::Result<ApplyResponse> {
        let mut successful_addresses = vec![];
        for change in request.changes {
            successful_addresses.push(change.address);
        }
        Ok(ApplyResponse {
            successful_addresses,
            failed_addresses: vec![],
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    
    let provider = ExampleProvider;
    let mut server = JsonRpcServer::new(provider);
    
    // Register schema for the 'user' resource type
    server.register_type_schema::<UserSpec>("user");
    
    eprintln!("Starting example-basic-provider JSON-RPC server on stdin/stdout...");
    server.run().await?;
    
    Ok(())
}
