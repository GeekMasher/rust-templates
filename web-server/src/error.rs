use rocket::http::Status;
use rocket::serde::json::Json;
/// Error handling
use thiserror::Error;

use crate::api::ApiError;

#[derive(Error, Debug)]
#[allow(unused)]
pub enum ServerError {
    /// Generic Internal Server Error
    #[error("Internal Server Error: {message}")]
    InternalServerError { message: String },

    /// Unauthorized Action
    #[error("Unauthorized: {message}")]
    Unauthorized {
        user_id: Option<i64>,
        action: String,
        message: String,
    },
    /// Unauthenticated Action
    #[error("Unauthenticated: {message}")]
    Unauthenticated {
        user_id: Option<i64>,
        action: String,
        message: String,
    },

    /// Database Connection Error
    #[error("Failed to connect to the database")]
    DatabaseConnectionError { message: String },
    /// ORM Error
    #[error("GeekOrm Error: {0}")]
    GeekOrmError(#[from] geekorm::Error),

    /// IO Error
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Responder)]
pub enum ApiErrorResponse {
    #[response(status = 401, content_type = "json")]
    Unauthorized { inner: (Status, Json<ApiError>) },
    #[response(status = 403, content_type = "json")]
    Forbidden { inner: (Status, Json<ApiError>) },
    #[response(status = 500, content_type = "json")]
    InternalServerError { inner: (Status, Json<ApiError>) },
}
