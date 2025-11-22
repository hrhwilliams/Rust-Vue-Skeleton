use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{database::DatabaseError, oauth::OAuthError};

#[derive(Serialize)]
struct ApiErrorResponse<'a> {
    message: &'a str,
    detail: Option<String>,
}

pub enum ApiError {
    BadRequest,
    DatabaseError(DatabaseError),
    OAuthError(String),
    NotFound,
    Unauthorized(Option<String>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.into()
    }
}

impl From<ApiError> for Response {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse {
                    message: "the request was malformed",
                    detail: None,
                }),
            ),
            ApiError::DatabaseError(error) => match error {
                DatabaseError::RngError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse {
                        message: "internal server error",
                        detail: Some("failed to generate random number".to_string()),
                    }),
                ),
                DatabaseError::SerdeError(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse {
                        message: "serialization error",
                        detail: Some(e.to_string()),
                    }),
                ),
                DatabaseError::SqlxError(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse {
                        message: "database error",
                        detail: Some(e.to_string()),
                    }),
                ),
            },
            ApiError::OAuthError(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    message: "oauth error",
                    detail: Some(detail),
                }),
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    message: "resource not found",
                    detail: None,
                }),
            ),
            ApiError::Unauthorized(detail) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    message: "you are not authorized to access this content",
                    detail,
                }),
            ),
        }
        .into_response()
    }
}

impl From<OAuthError> for ApiError {
    fn from(value: OAuthError) -> Self {
        match value {
            OAuthError::FailedToCreateAuthUrl => Self::OAuthError("failed to create auth URL".to_string()),
            OAuthError::FailedToStoreAttempt => Self::OAuthError("failed to store OAuth state".to_string()),
            OAuthError::FailedToRetrieveAttempt => Self::OAuthError("failed to retrieve OAuth state".to_string()),
            OAuthError::FailedToGetToken(reason) => Self::OAuthError(reason),
            OAuthError::FailedQuery => Self::OAuthError("failed to query with token".to_string()),
        }
    }
}

impl From<DatabaseError> for ApiError {
    fn from(value: DatabaseError) -> Self {
        Self::DatabaseError(value)
    }
}
