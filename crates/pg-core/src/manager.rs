use std::{collections::HashMap, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{debug, info, warn};

use crate::{
    config::DatabaseConfig,
    error::{PgError, Result},
};

/// Multi-database connection manager
pub struct DatabaseManager {
    connections: HashMap<String, DatabaseConnection>,
}

impl DatabaseManager {
    /// Create a new DatabaseManager with the given configurations
    pub async fn new(configs: Vec<DatabaseConfig>) -> Result<Self> {
        if configs.is_empty() {
            return Err(PgError::config(
                "At least one database configuration is required",
            ));
        }

        let mut connections = HashMap::new();

        for config in configs {
            info!("Connecting to database: {}", config.name);

            let mut opt = ConnectOptions::new(&config.url);
            opt.max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect_timeout(Duration::from_secs(config.connect_timeout))
                .idle_timeout(Duration::from_secs(config.idle_timeout))
                .sqlx_logging(config.sql_logging);

            let db = Database::connect(opt).await.map_err(|e| {
                PgError::internal(format!("Connection failed for {}: {}", config.name, e))
            })?;

            debug!("Successfully connected to database: {}", config.name);
            connections.insert(config.name.clone(), db);
        }

        Ok(Self { connections })
    }

    /// Get a database connection by name
    pub fn get(&self, name: &str) -> Result<&DatabaseConnection> {
        self.connections
            .get(name)
            .ok_or_else(|| PgError::not_found("Database", name))
    }

    /// Get the default database connection (first one added)
    pub fn default(&self) -> Result<&DatabaseConnection> {
        self.connections
            .values()
            .next()
            .ok_or_else(|| PgError::config("No database connections available"))
    }

    /// Get all database connection names
    pub fn list_databases(&self) -> Vec<&String> {
        self.connections.keys().collect()
    }

    /// Check if a database connection exists
    pub fn has_database(&self, name: &str) -> bool {
        self.connections.contains_key(name)
    }

    /// Get the number of database connections
    pub fn count(&self) -> usize {
        self.connections.len()
    }

    /// Close all database connections
    pub async fn close_all(&mut self) -> Result<()> {
        info!("Closing all database connections");

        for (name, conn) in self.connections.drain() {
            if let Err(e) = conn.close().await {
                warn!("Error closing database connection '{}': {}", name, e);
            } else {
                debug!("Closed database connection: {}", name);
            }
        }

        Ok(())
    }

    /// Close a specific database connection
    pub async fn close(&mut self, name: &str) -> Result<()> {
        if let Some(conn) = self.connections.remove(name) {
            info!("Closing database connection: {}", name);
            conn.close()
                .await
                .map_err(|e| PgError::internal(format!("Failed to close '{}': {}", name, e)))?;
            debug!("Closed database connection: {}", name);
            Ok(())
        } else {
            Err(PgError::not_found("Database", name))
        }
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        if !self.connections.is_empty() {
            warn!(
                "DatabaseManager dropped with {} active connections. Consider calling close_all() \
                 explicitly.",
                self.connections.len()
            );
        }
    }
}
