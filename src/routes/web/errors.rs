use axum::response::{Html, IntoResponse, Response};

use crate::{database::DatabaseError, oauth::OAuthError};

pub enum WebError {
    InternalServerError(String),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        self.into()
    }
}

impl From<WebError> for Response {
    fn from(value: WebError) -> Self {
        match value {
            WebError::InternalServerError(msg) => Html(format!(
                r#"<!doctype html><html lang="en"><body><p>Internal server error: {}</p></body></html>"#,
                msg
            ))
            .into_response(),
        }
    }
}

impl From<DatabaseError> for WebError {
    fn from(value: DatabaseError) -> Self {
        match value {
            DatabaseError::RngError => Self::InternalServerError("random number generator".to_string()),
            DatabaseError::SerdeError(e) => Self::InternalServerError(e.to_string()),
            DatabaseError::SqlxError(e) => Self::InternalServerError(e.to_string()),
        }
    }
}

impl From<OAuthError> for WebError {
    fn from(value: OAuthError) -> Self {
        match value {
            OAuthError::FailedQuery => Self::InternalServerError(String::new()),
            OAuthError::FailedToCreateAuthUrl => Self::InternalServerError(String::new()),
            OAuthError::FailedToGetToken(reason) => Self::InternalServerError(reason),
            OAuthError::FailedToRetrieveAttempt => Self::InternalServerError(String::new()),
            OAuthError::FailedToStoreAttempt => Self::InternalServerError(String::new()),
        }
    }
}
