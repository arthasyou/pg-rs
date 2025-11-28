use thiserror::Error;

/// Business logic and application-level errors
#[derive(Error, Debug)]
pub enum BusinessError {
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Business rule violation
    #[error("Business error: {0}")]
    RuleViolation(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl BusinessError {
    /// Create a permission denied error
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied(message.into())
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Create a business rule violation error
    pub fn rule_violation(message: impl Into<String>) -> Self {
        Self::RuleViolation(message.into())
    }

    /// Create a config error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Create a serialization error
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_denied() {
        let err = BusinessError::permission_denied("Access denied");
        assert_eq!(err.to_string(), "Permission denied: Access denied");
    }

    #[test]
    fn test_business_rule() {
        let err = BusinessError::rule_violation("Insufficient balance");
        assert_eq!(err.to_string(), "Business error: Insufficient balance");
    }
}
