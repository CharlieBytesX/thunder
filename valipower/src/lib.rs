// In my-validator/src/lib.rs
use std::collections::HashMap;
mod salvo_extractor;

// Represents a single validation error
#[derive(Debug)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
}

// A collection of all errors for a given struct validation
#[derive(Debug)]
pub struct ValidationErrors {
    // We can store errors per field if we want
    errors: std::collections::HashMap<String, Vec<ValidationError>>,
}

impl ValidationErrors {
    pub fn new() -> ValidationErrors {
        return ValidationErrors {
            errors: HashMap::new(),
        };
    }
    pub fn is_empty(&self) -> bool {
        return self.errors.is_empty();
    }
    pub fn add(&mut self, field_name: &str, error_type: &str, error_message: &str) {
        match self.errors.get_mut(field_name) {
            Some(value) => {
                value.push({
                    ValidationError {
                        code: error_type.to_string(),
                        message: error_message.to_string(),
                    }
                });
            }
            None => todo!(),
        }
    }
}
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

// Re-export the derive macro from the other crate
pub use valipower_macros::Validate;
