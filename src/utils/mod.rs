use crate::common::{types::ValidationResult, validate::validate_number};

pub fn fizzbuzz(x: f64) -> ValidationResult<String> {
    validate_number(x)?;
    let y = x as usize;
    let mut s: String = "".to_string();
    if y % 3 == 0 {
        s = "Fizz".to_string();
    }
    if y % 5 == 0 {
        s = format!("{}Buzz", s);
    }
    if s.is_empty() {
        return Ok(s);
    }
    Ok(format!("{y}"))
}
