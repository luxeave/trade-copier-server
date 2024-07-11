use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    PoolError(#[from] r2d2::Error),
    #[error("Internal server error")]
    InternalServerError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(_) => HttpResponse::InternalServerError().json("Database error occurred"),
            ApiError::PoolError(_) => HttpResponse::InternalServerError().json("Database connection pool error"),
            ApiError::InternalServerError(_) => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}

// Function to create an InternalServerError
pub fn internal_error<E: std::fmt::Debug>(err: E) -> ApiError {
    ApiError::InternalServerError(format!("{:?}", err))
}