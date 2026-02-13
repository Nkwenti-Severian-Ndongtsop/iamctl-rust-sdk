use iamctl_rust_sdk::prelude::*;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_resource_address_display() {
    let address = ResourceAddress {
        resource_type: "user".to_string(),
        name: "john-doe".to_string(),
        namespace: Some("prod".to_string()),
    };
    assert_eq!(address.to_string(), "user.prod/john-doe");

    let address_no_ns = ResourceAddress {
        resource_type: "user".to_string(),
        name: "john-doe".to_string(),
        namespace: None,
    };
    assert_eq!(address_no_ns.to_string(), "user.john-doe");
}

#[test]
fn test_resource_creation() {
    let address = ResourceAddress {
        resource_type: "user".to_string(),
        name: "john-doe".to_string(),
        namespace: Some("prod".to_string()),
    };

    let mut spec = HashMap::new();
    spec.insert("name".to_string(), json!("John Doe"));

    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "v1".to_string());

    let resource = Resource {
        address: address.clone(),
        spec: spec.clone(),
        metadata: metadata.clone(),
    };

    assert_eq!(resource.address, address);
    assert_eq!(resource.spec, spec);
    assert_eq!(resource.metadata, metadata);
}

#[test]
fn test_change_noop() {
    let address = ResourceAddress {
        resource_type: "user".to_string(),
        name: "john-doe".to_string(),
        namespace: None,
    };

    let change = Change {
        address: address.clone(),
        change_type: ChangeType::NoOp,
        before: None,
        after: None,
        computed_fields: vec![],
    };

    assert_eq!(change.change_type, ChangeType::NoOp);
}

#[test]
fn test_change_serialization() {
    let address = ResourceAddress {
        resource_type: "user".to_string(),
        name: "john-doe".to_string(),
        namespace: None,
    };

    let change = Change {
        address: address.clone(),
        change_type: ChangeType::Create,
        before: None,
        after: Some(Resource {
            address: address.clone(),
            spec: HashMap::new(),
            metadata: HashMap::new(),
        }),
        computed_fields: vec!["id".to_string()],
    };

    let serialized = serde_json::to_value(&change).unwrap();
    assert_eq!(serialized["change_type"], "Create");
    assert_eq!(serialized["computed_fields"][0], "id");
}
