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
            DatabaseError::RngError => WebError::InternalServerError("random number generator".to_string()),
            DatabaseError::SerdeError(e) => WebError::InternalServerError(e.to_string()),
            DatabaseError::SqlxError(e) => WebError::InternalServerError(e.to_string()),
        }
    }
}

impl From<OAuthError> for WebError {
    fn from(value: OAuthError) -> Self {
        match value {
            OAuthError::FailedQuery => WebError::InternalServerError("".to_string()),
            OAuthError::FailedToCreateAuthUrl => WebError::InternalServerError("".to_string()),
            OAuthError::FailedToGetToken(reason) => WebError::InternalServerError(reason),
            OAuthError::FailedToRetrieveAttempt => WebError::InternalServerError("".to_string()),
            OAuthError::FailedToStoreAttempt => WebError::InternalServerError("".to_string()),
        }
    }
}
