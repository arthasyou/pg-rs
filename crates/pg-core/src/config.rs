use serde::{Deserialize, Serialize};

/// Configuration for a single database connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Unique name for this database connection (e.g., "main", "replica", "analytics")
    pub name: String,

    /// Database connection URL (e.g., "postgres://user:pass@localhost/dbname")
    pub url: String,

    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of connections in the pool
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Connection timeout in seconds
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64,

    /// Idle timeout in seconds
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64,

    /// Enable SQL query logging
    #[serde(default = "default_sql_logging")]
    pub sql_logging: bool,
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    1
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_idle_timeout() -> u64 {
    600
}

fn default_sql_logging() -> bool {
    false
}

impl DatabaseConfig {
    /// Create a new database configuration with default settings
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            connect_timeout: default_connect_timeout(),
            idle_timeout: default_idle_timeout(),
            sql_logging: default_sql_logging(),
        }
    }

    /// Set the maximum number of connections
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Set the minimum number of connections
    pub fn min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// Set the connection timeout
    pub fn connect_timeout(mut self, timeout: u64) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set the idle timeout
    pub fn idle_timeout(mut self, timeout: u64) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// Enable SQL query logging
    pub fn with_sql_logging(mut self, enabled: bool) -> Self {
        self.sql_logging = enabled;
        self
    }
}
