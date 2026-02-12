use crate::types::{Change, Resource, ResourceAddress};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetadata {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub supported_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRequest {
    pub workspace_path: String,
    pub desired_state: Vec<Resource>,
    pub current_state: Vec<Resource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanResponse {
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyRequest {
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResponse {
    pub successful_addresses: Vec<ResourceAddress>,
    pub failed_addresses: Vec<(ResourceAddress, String)>,
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn metadata(&self) -> ProviderMetadata;
    fn capabilities(&self) -> ProviderCapabilities;
    
    async fn plan(&self, request: PlanRequest) -> anyhow::Result<PlanResponse>;
    async fn apply(&self, request: ApplyRequest) -> anyhow::Result<ApplyResponse>;
}
