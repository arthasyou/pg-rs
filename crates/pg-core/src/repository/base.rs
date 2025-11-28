use std::sync::Arc;

use sea_orm::DatabaseConnection;

/// Base repository implementation
pub struct BaseRepository {
    db: Arc<DatabaseConnection>,
}

impl BaseRepository {
    /// Create a new base repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
