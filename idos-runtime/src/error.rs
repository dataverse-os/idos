use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Builder, Clone, Default)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum AppErrorType {
    InternalServerError,
    DbError,
    NotFoundError,
    Unauthorized,
    InvalidQuery,

    Custom(StatusCode),
}

impl Default for AppErrorType {
    fn default() -> Self {
        AppErrorType::InternalServerError
    }
}

impl AppErrorType {
    pub fn err(&self) -> AppError {
        AppError {
            cause: None,
            message: None,
            error_type: self.clone(),
        }
    }
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            AppErrorType::InvalidQuery => StatusCode::BAD_REQUEST,

            AppErrorType::Custom(status_code) => status_code,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl AppError {
    pub fn new(status_code: StatusCode, message: &str) -> AppError {
        AppError {
            cause: None,
            message: Some(message.to_string()),
            error_type: AppErrorType::Custom(status_code),
        }
    }

    // we are handling the none. the function name should match the field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            AppError {
                cause: _,
                message: None,
                error_type: _,
            } => self.error_type.to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> AppError {
        AppError {
            cause: Some(error.to_string()),
            message: None,
            error_type: AppErrorType::InternalServerError,
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => AppError {
                cause: Some(value.to_string()),
                message: None,
                error_type: AppErrorType::NotFoundError,
            },
            _ => AppError {
                cause: Some(value.to_string()),
                message: None,
                error_type: AppErrorType::DbError,
            },
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(value: r2d2::Error) -> Self {
        AppError {
            cause: Some(value.to_string()),
            message: None,
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError {
            cause: Some(value.to_string()),
            message: None,
            error_type: AppErrorType::InternalServerError,
        }
    }
}
