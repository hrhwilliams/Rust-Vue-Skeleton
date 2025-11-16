use crate::errors::ApiError;

pub enum AuthError {
    MissingApiKey,
    MissingUserAgent,
    InvalidCredentials,
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingApiKey => ApiError::BadRequest,
            AuthError::MissingUserAgent => ApiError::BadRequest,
            AuthError::InvalidCredentials => ApiError::Unauthorized,
        }
    }
}
