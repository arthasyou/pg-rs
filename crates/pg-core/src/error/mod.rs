pub mod business;
pub mod database;
pub mod entity;
pub mod validation;

use sea_orm::DbErr;
use thiserror::Error;

/// Core error enum that encompasses all error types
#[derive(Error, Debug)]
pub enum PgError {
    /// Database errors
    #[error(transparent)]
    Database(#[from] database::DatabaseError),

    /// Database operation errors (direct DbErr conversion)
    #[error(transparent)]
    DatabaseOp(#[from] DbErr),

    /// Entity errors (CRUD operations)
    #[error(transparent)]
    Entity(#[from] entity::EntityError),

    /// Validation errors
    #[error(transparent)]
    Validation(#[from] validation::ValidationError),

    /// Validation errors (direct ValidationErrors conversion)
    #[error(transparent)]
    ValidationOp(#[from] validator::ValidationErrors),

    /// Business logic errors
    #[error(transparent)]
    Business(#[from] business::BusinessError),
}

impl PgError {
    // === Database error helpers ===

    /// Create a database connection error
    pub fn db_connection(message: impl Into<String>) -> Self {
        Self::Database(database::DatabaseError::connection(message))
    }

    /// Create a database not found error
    pub fn db_not_found(name: impl Into<String>) -> Self {
        Self::Database(database::DatabaseError::not_found(name))
    }

    // === Entity error helpers ===

    /// Create an entity not found error
    pub fn not_found(entity: impl Into<String>, id: impl std::fmt::Display) -> Self {
        Self::Entity(entity::EntityError::not_found(entity, id))
    }

    /// Create an entity already exists error
    pub fn already_exists(
        entity: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::Entity(entity::EntityError::already_exists(entity, field, value))
    }

    // === Validation error helpers ===

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(validation::ValidationError::single(message))
    }

    // === Business error helpers ===

    /// Create a permission denied error
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::Business(business::BusinessError::permission_denied(message))
    }

    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::Business(business::BusinessError::invalid_input(message))
    }

    /// Create a business rule violation error
    pub fn business(message: impl Into<String>) -> Self {
        Self::Business(business::BusinessError::rule_violation(message))
    }

    /// Create a config error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Business(business::BusinessError::config(message))
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Business(business::BusinessError::internal(message))
    }

    // === Checker methods ===

    /// Check if error is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::Entity(entity::EntityError::NotFound { .. }))
    }

    /// Check if error is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Check if error is a database error
    pub fn is_database(&self) -> bool {
        matches!(self, Self::Database(_))
    }

    /// Get validation errors if this is a Validation variant
    pub fn get_validation_errors(&self) -> Option<&validator::ValidationErrors> {
        match self {
            Self::Validation(validation::ValidationError::Multiple(errors)) => Some(errors),
            _ => None,
        }
    }
}

/// Backward compatibility alias

/// Result type alias using PgError
pub type Result<T> = std::result::Result<T, PgError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = PgError::not_found("User", 123);
        assert!(err.is_not_found());
        assert_eq!(err.to_string(), "Entity not found: User with id 123");
    }

    #[test]
    fn test_already_exists_error() {
        let err = PgError::already_exists("User", "email", "test@example.com");
        assert_eq!(
            err.to_string(),
            "Entity already exists: User with email=test@example.com"
        );
    }

    #[test]
    fn test_validation_error() {
        let err = PgError::validation("Invalid email format");
        assert!(err.is_validation());
        assert_eq!(err.to_string(), "Validation error: Invalid email format");
    }

    #[test]
    fn test_business_error() {
        let err = PgError::business("Insufficient balance");
        assert!(!err.is_validation());
        assert!(!err.is_not_found());
        assert_eq!(err.to_string(), "Business error: Insufficient balance");
    }

    #[test]
    fn test_config_error() {
        let err = PgError::config("Missing API key");
        assert_eq!(err.to_string(), "Configuration error: Missing API key");
    }

    #[test]
    fn test_database_error() {
        let err = PgError::db_connection("Connection timeout");
        assert!(err.is_database());
        assert_eq!(err.to_string(), "Connection error: Connection timeout");
    }
}
