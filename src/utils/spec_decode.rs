use serde::de::DeserializeOwned;

use crate::types::Resource;
use crate::utils::{Error, Result};

pub fn decode_spec<T: DeserializeOwned>(resource: &Resource) -> Result<T> {
    // Keep this intentionally simple and deterministic:
    // - convert spec map -> Value -> T
    // - attach resource address context to the error
    let value = serde_json::to_value(&resource.spec).map_err(Error::Serialization)?;
    serde_json::from_value::<T>(value).map_err(|e| {
        Error::Config(format!(
            "Invalid spec for {} (type={}): {}",
            resource.address, resource.address.resource_type, e
        ))
    })
}
