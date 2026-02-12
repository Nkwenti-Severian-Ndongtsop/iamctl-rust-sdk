use crate::types::Resource;
use crate::utils::Result;
use serde_json::Value;
use std::collections::HashMap;

/// Validation error details
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl JsonSchemaValidator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, resource_type: &str, schema: Value) {
        let _ = self.schemas.insert(resource_type.to_string(), schema);
    }

    fn validate_against_schema(&self, resource: &Resource) -> Result<ValidationResult> {
        let schema = match self.schemas.get(&resource.address.resource_type) {
            Some(schema) => schema,
            None => {
                return Ok(ValidationResult::invalid(vec![ValidationError::new(
                    "",
                    &format!("No schema found for resource type: {}", resource.address.resource_type),
                    "SCHEMA_NOT_FOUND",
                )]));
            }
        };

        // Basic validation logic - this will be expanded in later tasks
        self.validate_basic_constraints(resource, schema)
    }

    fn validate_basic_constraints(&self, resource: &Resource, _schema: &Value) -> Result<ValidationResult> {
        let mut errors = vec![];

        // Check if spec has required fields
        if resource.spec.is_empty() {
            errors.push(ValidationError::new(
                "spec",
                "Resource specification cannot be empty",
                "EMPTY_SPEC",
            ));
        }

        // Check for invalid field names (basic example)
        for key in resource.spec.keys() {
            if key.starts_with('_') {
                errors.push(ValidationError::new(
                    &format!("spec.{}", key),
                    "Field names cannot start with underscore",
                    "INVALID_FIELD_NAME",
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

impl CompositeValidator {
    pub fn new() -> Self {
        Self {
            validators: vec![],
        }
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
