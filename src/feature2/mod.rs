use crate::common::{types::ValidationResult, validate::validate_number};

pub fn bar(x: f64, y: f64) -> ValidationResult<f64> {
    validate_number(x)?;
    validate_number(y)?;
    Ok(x + y)
}
