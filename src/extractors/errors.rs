use axum::response::IntoResponse;

use crate::{
    database::DatabaseError,
    routes::{ApiError, WebError},
};

pub enum AuthError {
    MissingApiKey,
    MissingUserAgent,
    InvalidCredentials,
    DatabaseError(DatabaseError),
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingApiKey => Self::Unauthorized(Some("missing API key header".to_string())),
            AuthError::MissingUserAgent => Self::Unauthorized(Some("missing user agent header".to_string())),
            AuthError::InvalidCredentials => Self::Unauthorized(Some("API key was invalid".to_string())),
            AuthError::DatabaseError(e) => Self::from(e),
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
    NoSessionInDatabase,
    DatabaseError(DatabaseError),
}

impl From<SessionError> for WebError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::ExtractError => Self::InternalServerError("extract".to_string()),
            SessionError::NoSession => Self::InternalServerError("no session".to_string()),
            SessionError::NoSessionInDatabase => Self::InternalServerError("no session in db".to_string()),
            SessionError::DatabaseError(_e) => Self::InternalServerError("database error".to_string()),
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::ExtractError => Self::Unauthorized(Some("failed to read cookies".to_string())),
            SessionError::NoSession => Self::Unauthorized(Some("no session".to_string())),
            SessionError::NoSessionInDatabase => Self::Unauthorized(Some("no session in db".to_string())),
            SessionError::DatabaseError(e) => Self::from(e),
        }
    }
}

// impl IntoResponse for SessionError {
//     fn into_response(self) -> axum::response::Response {
//         WebError::from(self).into()
//     }
// }
