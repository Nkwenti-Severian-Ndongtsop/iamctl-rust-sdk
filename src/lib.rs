pub mod provider;
pub mod server;
pub mod types;
pub mod utils;
pub mod validation;

pub mod prelude {
    pub use crate::provider::{
        ApplyRequest, ApplyResponse, PlanRequest, PlanResponse, Provider, ProviderCapabilities,
        ProviderMetadata,
    };
    pub use crate::types::{Change, ChangeType, Resource, ResourceAddress};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
