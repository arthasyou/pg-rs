use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Database '{0}' not found")]
    DatabaseNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Sea-ORM error: {0}")]
    SeaOrmError(#[from] sea_orm::DbErr),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
