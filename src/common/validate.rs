use super::error::ValidationError;
use super::types::ValidationResult;

pub fn validate_number(n: f64) -> ValidationResult<()> {
    if n.is_nan() {
        return Err(ValidationError(format!("Expected Number: {}", n)));
    }
    if n.is_infinite() {
        return Err(ValidationError(format!("Expected Non-Infinite: {}", n)));
    }
    Ok(())
}
