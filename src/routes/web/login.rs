use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    PrivateCookieJar,
    cookie::Cookie,
};
use time::Duration;

use crate::{app::AppState, extractors::WebSession, routes::WebError};

#[debug_handler]
#[tracing::instrument(skip(app_state, session))]
pub async fn login(
    State(app_state): State<AppState>,
    WebSession(mut session): WebSession,
    jar: PrivateCookieJar,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, WebError> {
    if let Some(invite_code) = query.get("invite") {
        session.set("invite", invite_code).await?;
    }

    let (url, token, verifier) = app_state.oauth.get_oauth_url()?;

    let csrf_cookie = Cookie::build(("csrf_token", token.into_secret()))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::seconds(60));
    let verifier_cookie = Cookie::build(("verifier", verifier.into_secret()))
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::seconds(60));

    Ok((jar.add(csrf_cookie).add(verifier_cookie), Redirect::to(&url)))
}
