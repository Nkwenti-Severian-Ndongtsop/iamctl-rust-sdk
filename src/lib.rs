pub mod types;
pub mod provider;
pub mod server;
pub mod validation;
pub mod utils;

pub mod prelude {
    pub use crate::types::{Change, ChangeType, Resource, ResourceAddress};
    pub use crate::provider::{
        ApplyRequest, ApplyResponse, PlanRequest, PlanResponse, Provider, ProviderCapabilities,
        ProviderMetadata,
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
