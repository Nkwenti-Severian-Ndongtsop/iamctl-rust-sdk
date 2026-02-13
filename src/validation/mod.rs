use crate::types::Resource;
use crate::utils::Result;
use jsonschema::JSONSchema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Validation error details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub code: String,
}

impl ValidationError {
    pub fn new(path: &str, message: &str, code: &str) -> Self {
        Self {
            path: path.to_string(),
            message: message.to_string(),
            code: code.to_string(),
        }
    }
}

/// Validation result containing errors and warnings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: vec![],
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<ValidationError>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// Schema validator for resource specifications
pub trait SchemaValidator: Send + Sync {
    fn validate(&self, resource: &Resource) -> Result<ValidationResult>;
}

/// JSON Schema validator implementation
pub struct JsonSchemaValidator {
    schemas: HashMap<String, Value>,
}

impl Default for JsonSchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonSchemaValidator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, resource_type: &str, schema: Value) {
        let _ = self.schemas.insert(resource_type.to_string(), schema);
    }

    /// Helper to add a schema by deriving it from a Rust type
    pub fn add_type_schema<T: JsonSchema>(&mut self, resource_type: &str) {
        let schema = schemars::schema_for!(T);
        match serde_json::to_value(&schema) {
            Ok(schema_value) => self.add_schema(resource_type, schema_value),
            Err(e) => {
                tracing::warn!(resource_type, error = %e, "Failed to serialize derived JSON schema");
            }
        }
    }

    fn validate_against_schema(&self, resource: &Resource) -> Result<ValidationResult> {
        let schema_value = match self.schemas.get(&resource.address.resource_type) {
            Some(schema) => schema,
            None => {
                return Ok(ValidationResult::invalid(vec![ValidationError::new(
                    "",
                    &format!(
                        "No schema found for resource type: {}",
                        resource.address.resource_type
                    ),
                    "SCHEMA_NOT_FOUND",
                )]));
            }
        };

        let mut errors = vec![];

        // 1. Basic internal validation (can be removed if redundant with schema)
        if resource.spec.is_empty() {
            errors.push(ValidationError::new(
                "spec",
                "Resource specification cannot be empty",
                "EMPTY_SPEC",
            ));
        }

        // 2. Full JSON Schema validation
        let compiled = match JSONSchema::compile(schema_value) {
            Ok(s) => s,
            Err(e) => {
                return Ok(ValidationResult::invalid(vec![ValidationError::new(
                    "",
                    &format!("Invalid JSON Schema: {e}"),
                    "INVALID_SCHEMA_DEFINITION",
                )]));
            }
        };

        let spec_value = serde_json::to_value(&resource.spec).unwrap_or(Value::Null);
        if let Err(schema_errors) = compiled.validate(&spec_value) {
            for error in schema_errors {
                errors.push(ValidationError::new(
                    &format!("spec{}", error.instance_path),
                    &error.to_string(),
                    "SCHEMA_VALIDATION_ERROR",
                ));
            }
        }

        if errors.is_empty() {
            Ok(ValidationResult::valid())
        } else {
            Ok(ValidationResult::invalid(errors))
        }
    }
}

impl SchemaValidator for JsonSchemaValidator {
    fn validate(&self, resource: &Resource) -> Result<ValidationResult> {
        self.validate_against_schema(resource)
    }
}

/// Composite validator that runs multiple validators
pub struct CompositeValidator {
    validators: Vec<Box<dyn SchemaValidator>>,
}

impl Default for CompositeValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositeValidator {
    pub fn new() -> Self {
        Self { validators: vec![] }
    }

    pub fn add_validator(mut self, validator: Box<dyn SchemaValidator>) -> Self {
        self.validators.push(validator);
        self
    }
}

impl SchemaValidator for CompositeValidator {
    fn validate(&self, resource: &Resource) -> Result<ValidationResult> {
        let mut all_errors = vec![];
        let mut all_warnings = vec![];
        let mut valid = true;

        for validator in &self.validators {
            let result = validator.validate(resource)?;
            if !result.valid {
                valid = false;
            }
            all_errors.extend(result.errors);
            all_warnings.extend(result.warnings);
        }

        Ok(ValidationResult {
            valid,
            errors: all_errors,
            warnings: all_warnings,
        })
    }
}
