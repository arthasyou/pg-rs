use thiserror::Error;
use validator::ValidationErrors;

/// Validation-related errors
#[derive(Error, Debug)]
pub enum ValidationError {
    /// Single field validation error
    #[error("Validation error: {0}")]
    Single(String),

    /// Multiple field validation errors
    #[error("Validation failed: {0}")]
    Multiple(#[from] ValidationErrors),
}

impl ValidationError {
    /// Create a single validation error
    pub fn single(message: impl Into<String>) -> Self {
        Self::Single(message.into())
    }

    /// Get validator errors if this is a Multiple variant
    pub fn get_errors(&self) -> Option<&ValidationErrors> {
        match self {
            Self::Multiple(errors) => Some(errors),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_validation_error() {
        let err = ValidationError::single("Invalid email format");
        assert_eq!(err.to_string(), "Validation error: Invalid email format");
    }
}
