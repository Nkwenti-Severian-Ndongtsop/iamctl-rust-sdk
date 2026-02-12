use iamctl_rust_sdk::prelude::*;
use async_trait::async_trait;

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "mock-provider".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supported_resources: vec!["realm".to_string(), "client".to_string()],
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    iamctl_rust_sdk::utils::init_logging();
    
    let provider = MockProvider;
    println!("Provider: {} (v{})", provider.metadata().name, provider.metadata().version);
    
    // In a real implementation, we would start the JSON-RPC server here
    // let server = JsonRpcServer::new(provider);
    // server.serve().await?;
    
    Ok(())
}
