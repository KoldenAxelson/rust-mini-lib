use std::fmt;

#[derive(Debug, Clone)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValidationError: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}
