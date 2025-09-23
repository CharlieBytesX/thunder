// In my-validator/src/lib.rs

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
    // Methods to add errors, check if empty, etc.
    // ...
}
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

// Re-export the derive macro from the other crate
pub use valipower_macros::Validate;
