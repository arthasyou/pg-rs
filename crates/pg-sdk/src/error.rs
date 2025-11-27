use sea_orm::DbErr;
use thiserror::Error;
use validator::ValidationErrors;

/// SDK error types
#[derive(Error, Debug)]
pub enum SdkError {
    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] DbErr),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Validation errors (multiple fields)
    #[error("Validation failed: {0}")]
    ValidationErrors(#[from] ValidationErrors),

    /// Entity not found
    #[error("Entity not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    /// Entity already exists
    #[error("Entity already exists: {entity} with {field}={value}")]
    AlreadyExists {
        entity: String,
        field: String,
        value: String,
    },

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Business logic error
    #[error("Business error: {0}")]
    Business(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Result type alias using SdkError
pub type Result<T> = std::result::Result<T, SdkError>;

impl SdkError {
    /// Create a not found error
    pub fn not_found(entity: impl Into<String>, id: impl std::fmt::Display) -> Self {
        Self::NotFound {
            entity: entity.into(),
            id: id.to_string(),
        }
    }

    /// Create an already exists error
    pub fn already_exists(
        entity: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::AlreadyExists {
            entity: entity.into(),
            field: field.into(),
            value: value.into(),
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a permission denied error
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied(message.into())
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Create a business error
    pub fn business(message: impl Into<String>) -> Self {
        Self::Business(message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Check if error is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }

    /// Check if error is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_) | Self::ValidationErrors(_))
    }

    /// Check if error is a database error
    pub fn is_database(&self) -> bool {
        matches!(self, Self::Database(_))
    }

    /// Get validation errors if this is a ValidationErrors variant
    pub fn get_validation_errors(&self) -> Option<&ValidationErrors> {
        match self {
            Self::ValidationErrors(errors) => Some(errors),
            _ => None,
        }
    }
}

/// Convert DbErr RecordNotFound to SdkError
pub fn db_not_found_to_sdk(
    entity: impl Into<String>,
    id: impl std::fmt::Display,
) -> impl Fn(DbErr) -> SdkError {
    let entity = entity.into();
    let id = id.to_string();
    move |err| match err {
        DbErr::RecordNotFound(_) => SdkError::not_found(&entity, &id),
        other => SdkError::Database(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = SdkError::not_found("User", 123);
        assert!(err.is_not_found());
        assert_eq!(err.to_string(), "Entity not found: User with id 123");
    }

    #[test]
    fn test_already_exists_error() {
        let err = SdkError::already_exists("User", "email", "test@example.com");
        assert_eq!(
            err.to_string(),
            "Entity already exists: User with email=test@example.com"
        );
    }

    #[test]
    fn test_validation_error() {
        let err = SdkError::validation("Invalid email format");
        assert!(err.is_validation());
        assert_eq!(err.to_string(), "Validation error: Invalid email format");
    }

    #[test]
    fn test_error_helpers() {
        let err = SdkError::business("Insufficient balance");
        assert!(!err.is_validation());
        assert!(!err.is_not_found());
        assert_eq!(err.to_string(), "Business error: Insufficient balance");
    }
}
