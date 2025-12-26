mod config;
mod error;
mod manager;

pub mod query;
pub mod repository;

// Re-export core utilities
// pg-tables/src/db.rs
use std::sync::Arc;

pub use config::DatabaseConfig;
pub use error::{PgError, Result};
pub use manager::DatabaseManager;
pub use query::{OrderBy, PaginatedResponse, PaginationParams};
pub use repository::{Repository, base::BaseRepository};
use sea_orm::{Database, DatabaseConnection};

/// pg-tables 对外暴露的数据库上下文
///
/// 约定：
/// - 外部 crate 只能“拿着它用”
/// - 不能依赖 SeaORM
#[derive(Clone)]
pub struct DbContext {
    inner: Arc<DatabaseConnection>,
}

impl DbContext {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { inner: db }
    }

    pub(crate) fn inner(&self) -> &DatabaseConnection {
        &self.inner
    }
}

/// ===============================
/// 对外唯一入口：创建 DbContext
/// ===============================

/// 创建数据库上下文（pg-tables 统一入口）
///
/// - 外部只提供连接字符串
/// - SeaORM 只存在于 pg-tables 内
pub async fn create_db_context(database_url: &str) -> Result<DbContext> {
    let conn = Database::connect(database_url).await?;
    Ok(DbContext::new(Arc::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = DatabaseConfig::new("test", "postgres://localhost/test")
            .max_connections(20)
            .min_connections(5)
            .connect_timeout(60)
            .with_sql_logging(true);

        assert_eq!(config.name, "test");
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.connect_timeout, 60);
        assert_eq!(config.sql_logging, true);
    }

    #[test]
    fn test_config_defaults() {
        let config = DatabaseConfig::new("test", "postgres://localhost/test");

        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 1);
        assert_eq!(config.connect_timeout, 30);
        assert_eq!(config.idle_timeout, 600);
        assert_eq!(config.sql_logging, false);
    }
}
