use sea_orm::DbErr;
use thiserror::Error;

/// Database-related errors
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// Database operation error
    #[error("Database error: {0}")]
    Operation(#[from] DbErr),

    /// Database connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Database not found
    #[error("Database not found: {0}")]
    NotFound(String),
}

impl DatabaseError {
    /// Create a connection error
    pub fn connection(message: impl Into<String>) -> Self {
        Self::Connection(message.into())
    }

    /// Create a not found error
    pub fn not_found(name: impl Into<String>) -> Self {
        Self::NotFound(name.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_error() {
        let err = DatabaseError::connection("Connection timeout");
        assert_eq!(err.to_string(), "Connection error: Connection timeout");
    }

    #[test]
    fn test_not_found_error() {
        let err = DatabaseError::not_found("main");
        assert_eq!(err.to_string(), "Database not found: main");
    }
}
