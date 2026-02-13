use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::validation::{CompositeValidator, JsonSchemaValidator};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_json_schema_validator_full_validation() {
    let mut validator = JsonSchemaValidator::new();
    let schema = json!({
        "type": "object",
        "properties": {
            "age": { "type": "integer", "minimum": 18 }
        },
        "required": ["age"]
    });

    validator.add_schema("user", schema);

    // Test valid resource
    let mut spec = HashMap::new();
    spec.insert("age".to_string(), json!(20));
    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test".to_string(),
            namespace: None,
        },
        spec,
        metadata: HashMap::new(),
    };

    let result = validator.validate(&resource).unwrap();
    assert!(result.valid);

    // Test invalid resource (age too young)
    let mut spec = HashMap::new();
    spec.insert("age".to_string(), json!(15));
    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test".to_string(),
            namespace: None,
        },
        spec,
        metadata: HashMap::new(),
    };

    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == "SCHEMA_VALIDATION_ERROR"));
}

#[test]
fn test_json_schema_validator_invalid_schema() {
    let mut validator = JsonSchemaValidator::new();
    // Invalid schema (type must be a string or array)
    let schema = json!({ "type": 123 });

    validator.add_schema("user", schema);

    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test".to_string(),
            namespace: None,
        },
        spec: HashMap::new(),
        metadata: HashMap::new(),
    };

    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
    assert_eq!(result.errors[0].code, "INVALID_SCHEMA_DEFINITION");
}

#[test]
fn test_composite_validator_all_rules() {
    let mut composite = CompositeValidator::new();
    let mut json_validator = JsonSchemaValidator::new();

    json_validator.add_schema(
        "user",
        json!({
            "type": "object",
            "required": ["email"]
        }),
    );

    composite = composite.add_validator(Box::new(json_validator));

    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test".to_string(),
            namespace: None,
        },
        spec: HashMap::new(), // Fails both EMPTY_SPEC and schema required email
        metadata: HashMap::new(),
    };

    let result = composite.validate(&resource).unwrap();
    assert!(!result.valid);
    // Should have multiple errors
    assert!(result.errors.len() >= 2);
}
