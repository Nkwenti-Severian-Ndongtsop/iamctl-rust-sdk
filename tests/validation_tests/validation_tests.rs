use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::validation::{CompositeValidator, JsonSchemaValidator};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_validation_error_creation() {
    let error = ValidationError::new("spec.field", "Invalid field", "INVALID_FIELD");

    assert_eq!(error.path, "spec.field");
    assert_eq!(error.message, "Invalid field");
    assert_eq!(error.code, "INVALID_FIELD");
}

#[test]
fn test_validation_result_valid() {
    let result = ValidationResult::valid();

    assert!(result.valid);
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_validation_result_invalid() {
    let errors = vec![
        ValidationError::new("spec", "Empty spec", "EMPTY_SPEC"),
        ValidationError::new("spec.field", "Invalid name", "INVALID_NAME"),
    ];
    let result = ValidationResult::invalid(errors.clone());

    assert!(!result.valid);
    assert_eq!(result.errors.len(), errors.len());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_json_schema_validator_new() {
    let validator = JsonSchemaValidator::new();

    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test-user".to_string(),
            namespace: None,
        },
        spec: HashMap::new(),
        metadata: HashMap::new(),
    };
    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
}

#[test]
fn test_json_schema_validator_add_schema() {
    let mut validator = JsonSchemaValidator::new();
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"}
        }
    });

    validator.add_schema("user", schema);

    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test-user".to_string(),
            namespace: None,
        },
        spec: HashMap::new(),
        metadata: HashMap::new(),
    };
    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
    assert_eq!(result.errors[0].code, "EMPTY_SPEC");
}

#[test]
fn test_composite_validator_new() {
    let validator = CompositeValidator::new();
    let resource = Resource {
        address: ResourceAddress {
            resource_type: "user".to_string(),
            name: "test-user".to_string(),
            namespace: None,
        },
        spec: {
            let mut spec = HashMap::new();
            spec.insert("name".to_string(), json!("John"));
            spec
        },
        metadata: HashMap::new(),
    };

    let result = validator.validate(&resource).unwrap();
    assert!(result.valid);
}
