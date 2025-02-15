use rocket::http::Status;
use rocket::response::Responder;
/// # API
use rocket::serde::json::Json;
use rocket::{response, Request};

use crate::error::{ApiErrorResponse, ServerError};

/// API Result which is a JSON response or a ServerError
pub type ApiResult<T> = Result<Json<T>, ServerError>;

/// API Error Response
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub status: i16,
}

/// Convert a ServerError to an ApiError
impl<'r> Responder<'r, 'r> for ServerError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'r> {
        match self {
            // Unauthorized
            ServerError::Unauthorized {
                ref user_id,
                ref action,
                ref message,
            } => {
                log::error!(
                    "Unauthorized: user_id: {:?}, action: {}, message: {}",
                    user_id,
                    action,
                    message
                );
                ApiErrorResponse::Unauthorized {
                    inner: (
                        Status::Unauthorized,
                        Json(ApiError {
                            message: "Unauthorized".to_string(),
                            details: Some(self.to_string()),
                            status: 401,
                        }),
                    ),
                }
            }
            // Forbidden
            ServerError::Unauthenticated {
                ref user_id,
                ref action,
                ref message,
            } => {
                log::error!(
                    "Unauthenticated: user_id: {:?}, action: {}, message: {}",
                    user_id,
                    action,
                    message
                );
                ApiErrorResponse::Forbidden {
                    inner: (
                        Status::Forbidden,
                        Json(ApiError {
                            message: "Unauthenticated".to_string(),
                            details: Some(self.to_string()),
                            status: 403,
                        }),
                    ),
                }
            }
            // Internal Server Error (default)
            _ => ApiErrorResponse::InternalServerError {
                inner: (
                    Status::InternalServerError,
                    Json(ApiError {
                        message: "Internal Server Error".to_string(),
                        details: Some(self.to_string()),
                        status: 500,
                    }),
                ),
            },
        }
        .respond_to(request)
    }
}
