use std::sync::OnceLock;

use pg_core::{DatabaseConfig, DatabaseManager, DbContext};

use crate::error::{Error, Result};

static DB_MANAGER: OnceLock<DatabaseManager> = OnceLock::new();

pub async fn init_db(configs: Vec<DatabaseConfig>) -> Result<()> {
    let manager = DatabaseManager::new(configs)
        .await
        .map_err(|e| Error::Custom(format!("DatabaseManager init failed: {e}")))?;

    DB_MANAGER
        .set(manager)
        .map_err(|_| Error::Custom("DatabaseManager already initialized".to_owned()))?;
    Ok(())
}

fn get_db_manager() -> &'static DatabaseManager {
    DB_MANAGER.get().expect("DatabaseManager not initialized")
}

pub fn get_default_ctx() -> DbContext {
    get_db_manager().default()
}

pub fn get_specific_ctx(name: &str) -> Result<DbContext> {
    let ctx = get_db_manager()
        .get(name)
        .map_err(|_| Error::NotFound(format!("database '{name}' not found")))?;
    Ok(ctx)
}
