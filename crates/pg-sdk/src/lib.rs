// Re-export pg-core for convenience
pub use pg_core;
// Re-export core utilities from pg-core
pub use pg_core::{
    BaseRepository, PaginatedResponse, PaginationParams, PgError as SdkError, Repository, Result,
};

// Entity definitions
pub mod entity;

pub mod error;
pub mod table;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Test that pg_core is accessible
        let _config = pg_core::DatabaseConfig::new("test", "postgres://localhost/test");
    }
}
