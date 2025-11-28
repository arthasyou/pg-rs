use thiserror::Error;

/// Entity-related errors (CRUD operations)
#[derive(Error, Debug)]
pub enum EntityError {
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

    /// Invalid entity state
    #[error("Invalid entity state: {0}")]
    InvalidState(String),
}

impl EntityError {
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

    /// Create an invalid state error
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into())
    }

    /// Check if error is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = EntityError::not_found("User", 123);
        assert!(err.is_not_found());
        assert_eq!(err.to_string(), "Entity not found: User with id 123");
    }

    #[test]
    fn test_already_exists_error() {
        let err = EntityError::already_exists("User", "email", "test@example.com");
        assert_eq!(
            err.to_string(),
            "Entity already exists: User with email=test@example.com"
        );
    }
}
