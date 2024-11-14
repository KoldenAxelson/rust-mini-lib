use super::error::ValidationError;

pub type ValidationResult<T> = Result<T, ValidationError>;
