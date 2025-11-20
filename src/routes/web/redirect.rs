use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::PkceCodeVerifier;

use crate::{app::AppState, extractors::WebSession, routes::WebError};

#[tracing::instrument(skip(app_state, session))]
pub async fn redirect(
    State(app_state): State<AppState>,
    WebSession(mut session): WebSession,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, WebError> {
    let code = query
        .get("code")
        .ok_or_else(|| WebError::InternalServerError("missing code".to_string()))?;
    let state = query
        .get("state")
        .ok_or_else(|| WebError::InternalServerError("missing state".to_string()))?;

    let csrf_token = session
        .get::<String>("csrf_token")
        .map_err(|e| WebError::InternalServerError(e.to_string()))?
        .ok_or_else(|| WebError::InternalServerError("missing csrf_token".to_string()))?;

    if state != &csrf_token {
        return Err(WebError::InternalServerError("state and csrf token do not match".to_string()));
    }

    let pkce_verifier = session
        .get::<PkceCodeVerifier>("verifier")
        .map_err(|e| WebError::InternalServerError(e.to_string()))?;

    let token = app_state
        .oauth
        .get_token(
            pkce_verifier.ok_or_else(|| WebError::InternalServerError("missing verifier".to_string()))?,
            code,
        )
        .await?;

    session.remove("csrf_token").await?;
    session.remove("verifier").await?;
    session.set("token", token).await?;

    Ok(Redirect::to("/"))
}
