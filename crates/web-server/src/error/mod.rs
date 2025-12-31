pub mod error_code;

use axum::http::StatusCode;
use pg_core::ErrorKind;
use thiserror::Error;
use toolcraft_axum_kit::{ApiError, CommonError};

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("config error: {0}")]
    #[allow(clippy::enum_variant_names)]
    Config(#[from] toolcraft_config::error::Error),

    #[error("validation error: {0}")]
    #[allow(clippy::enum_variant_names)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[allow(clippy::enum_variant_names)]
    Core(#[from] pg_core::Error),

    #[error("custom error: {0}")]
    Custom(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        error_to_api_error(err)
    }
}

fn error_to_api_error(err: Error) -> ApiError {
    match err {
        Error::Validation(e) => (
            StatusCode::BAD_REQUEST,
            CommonError {
                code: 400,
                message: e.to_string(),
            }
            .to_json(),
        ),
        Error::Config(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            CommonError {
                code: 500,
                message: format!("config error: {}", e),
            }
            .to_json(),
        ),
        Error::Core(e) => map_core_error(e),
        Error::Custom(_) => todo!(),
    }
}

fn map_core_error(err: pg_core::Error) -> ApiError {
    match err.kind() {
        ErrorKind::NotFound => (
            StatusCode::NOT_FOUND,
            CommonError {
                code: 404,
                message: err.to_string(),
            }
            .to_json(),
        ),

        ErrorKind::Validation => (
            StatusCode::BAD_REQUEST,
            CommonError {
                code: 400,
                message: err.to_string(),
            }
            .to_json(),
        ),

        ErrorKind::Permission => (
            StatusCode::FORBIDDEN,
            CommonError {
                code: 403,
                message: err.to_string(),
            }
            .to_json(),
        ),

        ErrorKind::Conflict => (
            StatusCode::CONFLICT,
            CommonError {
                code: 409,
                message: err.to_string(),
            }
            .to_json(),
        ),

        ErrorKind::Database | ErrorKind::Internal => (
            StatusCode::INTERNAL_SERVER_ERROR,
            CommonError {
                code: 500,
                message: "Internal Server Error".to_string(),
            }
            .to_json(),
        ),
    }
}
