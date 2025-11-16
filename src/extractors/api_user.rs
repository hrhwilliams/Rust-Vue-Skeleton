use axum::extract::FromRequestParts;
use axum::http::{header::USER_AGENT, request::Parts};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::database::ApiUserModel;
use crate::errors::ApiError;
use crate::extractors::errors::AuthError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedApiUser(pub String);

impl FromRequestParts<AppState> for AuthenticatedApiUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let api_key = parts
            .headers
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .ok_or(ApiError::from(AuthError::MissingApiKey))?;

        let user_agent = parts
            .headers
            .get(USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .ok_or(ApiError::from(AuthError::MissingUserAgent))?
            .to_string();

        let has_api_key = state
            .db
            .validate_api_key(api_key)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        // .ok_or(ApiError::from(AuthError::InvalidCredentials))
        // .inspect_err(|_| {
        //     tracing::info!("Login attempt by IP: '{}' via User-Agent '{}'", ip, user_agent);
        // })?;

        // tracing::Span::current().record("user_agent", tracing::field::display(&user_record.user_agent));

        if has_api_key {
            Ok(AuthenticatedApiUser(user_agent.to_string()))
        } else {
            Err(ApiError::from(AuthError::InvalidCredentials))
        }
    }
}
