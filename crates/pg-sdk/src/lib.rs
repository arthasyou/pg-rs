// Re-export pg-core for convenience
pub use pg_core;
// Re-export sea_orm for convenience
pub use sea_orm;

// Error types
pub mod error;
pub use error::{Result, SdkError};

// Entity definitions
pub mod entity;

// Core utilities (pagination, repository, validation)
pub mod core;

// Business logic modules
pub mod prompt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Test that pg_core is accessible
        let _config = pg_core::DatabaseConfig::new("test", "postgres://localhost/test");
    }
}
