// Re-export pg-core for convenience
pub use pg_core;
// Re-export core utilities from pg-core
pub use pg_core::{
    BaseRepository, PaginatedResponse, PaginationParams, PgError as SdkError, Repository, Result,
    Validate, ValidationError, ValidationErrors, format_validation_errors, validate_struct,
};

// Entity definitions
pub mod entity;

// Business logic modules - domain layer
pub mod domain;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Test that pg_core is accessible
        let _config = pg_core::DatabaseConfig::new("test", "postgres://localhost/test");
    }
}
