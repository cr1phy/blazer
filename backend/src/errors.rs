use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum ServerError {
    #[error("Internal error")]
    InternalError,
    #[error("User already exists")]
    UserFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status = match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::UserFound => StatusCode::CONFLICT,
            ServerError::UserNotFound
            | ServerError::InvalidCredentials
            | ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        HttpResponse::build(status).json(self)
    }
}
