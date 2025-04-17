// src/domain/error.rs  (new file)
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Todo not found")]
    NotFound,
    #[error("Unexpected repository error")]
    Repo(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl ResponseError for TodoError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TodoError::NotFound => HttpResponse::NotFound().finish(),
            TodoError::Repo(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
