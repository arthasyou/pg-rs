// Re-export pg-core for convenience
pub use pg_core;
// Re-export sea_orm for convenience
pub use sea_orm;

// Business logic modules will go here
// pub mod prompt;
// pub mod user;
// etc.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Test that pg_core is accessible
        let _config = pg_core::DatabaseConfig::new("test", "postgres://localhost/test");
    }
}
