pub mod error_code;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("config error: {0}")]
    #[allow(clippy::enum_variant_names)]
    ConfigError(#[from] toolcraft_config::error::Error),

    #[error("validation error: {0}")]
    #[allow(clippy::enum_variant_names)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("not found: {0}")]
    NotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::ConfigError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::ValidationError(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::NotFound(ref e) => (StatusCode::NOT_FOUND, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
