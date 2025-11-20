use axum::response::IntoResponse;

use crate::{
    database::DatabaseError,
    routes::{ApiError, WebError},
};

pub enum AuthError {
    MissingApiKey,
    MissingUserAgent,
    InvalidCredentials,
    DatabaseError(String),
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingApiKey => ApiError::Unauthorized(Some("missing API key header".to_string())),
            AuthError::MissingUserAgent => ApiError::Unauthorized(Some("missing user agent header".to_string())),
            AuthError::InvalidCredentials => ApiError::Unauthorized(Some("API key was invalid".to_string())),
            AuthError::DatabaseError(s) => ApiError::DatabaseError(s),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        ApiError::from(self).into()
    }
}

pub enum SessionError {
    ExtractError,
    NoSession,
    DatabaseError(DatabaseError),
}

impl From<SessionError> for WebError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::ExtractError => WebError::InternalServerError("extract".to_string()),
            SessionError::NoSession => WebError::InternalServerError("no session".to_string()),
            SessionError::DatabaseError(_e) => WebError::InternalServerError("database error".to_string()),
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::ExtractError => ApiError::Unauthorized(Some("failed to read cookies".to_string())),
            SessionError::NoSession => ApiError::Unauthorized(Some("no session".to_string())),
            SessionError::DatabaseError(_e) => ApiError::DatabaseError("database error".to_string()),
        }
    }
}

// impl IntoResponse for SessionError {
//     fn into_response(self) -> axum::response::Response {
//         WebError::from(self).into()
//     }
// }
