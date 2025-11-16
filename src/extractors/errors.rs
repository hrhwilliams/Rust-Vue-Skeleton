use crate::errors::ApiError;

pub enum AuthError {
    MissingApiKey,
    MissingUserAgent,
    InvalidCredentials,
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingApiKey => ApiError::Unauthorized(Some("missing API key header".to_string())),
            AuthError::MissingUserAgent => ApiError::Unauthorized(Some("missing user agent header".to_string())),
            AuthError::InvalidCredentials => ApiError::Unauthorized(Some("API key was invalid".to_string())),
        }
    }
}
