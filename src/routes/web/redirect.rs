use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};
use oauth2::PkceCodeVerifier;
use time::Duration;

use crate::{app::AppState, extractors::WebSession, routes::WebError};

#[tracing::instrument(skip(jar))]
pub async fn redirect(
    jar: PrivateCookieJar,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, WebError> {
    let code = query
        .get("code")
        .ok_or_else(|| WebError::InternalServerError("missing code".to_string()))?;
    let state = query
        .get("state")
        .ok_or_else(|| WebError::InternalServerError("missing state".to_string()))?;

    let csrf_token = jar
        .get("csrf_token")
        .ok_or_else(|| WebError::InternalServerError("missing csrf_token".to_string()))?
        .value()
        .to_string();

    if state != &csrf_token {
        return Err(WebError::InternalServerError(
            "state and csrf token do not match".to_string(),
        ));
    }

    let token_cookie = Cookie::build(("discord_token", code.clone()))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::seconds(60));

    Ok((
        jar.remove("csrf_token")
            .add(token_cookie),
        Redirect::to("/oauth/finalize"),
    ))
}

pub async fn finalize(
    State(app_state): State<AppState>,
    WebSession(mut session): WebSession,
    jar: PrivateCookieJar,
) -> Result<impl IntoResponse, WebError> {
    let pkce_verifier = jar
        .get("verifier")
        .ok_or_else(|| WebError::InternalServerError("missing csrf_token".to_string()))?
        .value()
        .to_string();

    let code = jar
        .get("discord_token")
        .ok_or_else(|| WebError::InternalServerError("missing csrf_token".to_string()))?
        .value()
        .to_string();

    let token = app_state
        .oauth
        .get_token(PkceCodeVerifier::new(pkce_verifier), &code)
        .await?;

    session.set("token", token).await?;

    Ok((jar.remove("verifier").remove("discord_token"), Redirect::to("/")))
}
