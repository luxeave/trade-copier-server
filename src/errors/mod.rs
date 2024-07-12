use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    PoolError(#[from] r2d2::Error),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e)),
            ApiError::PoolError(e) => HttpResponse::InternalServerError().json(format!("Pool error: {}", e)),
            ApiError::InternalServerError(e) => HttpResponse::InternalServerError().json(format!("Internal error: {}", e)),
        }
    }
}

// You can keep your internal_error function here as well
pub fn internal_error<E: std::fmt::Debug>(err: E) -> ApiError {
    ApiError::InternalServerError(format!("{:?}", err))
}