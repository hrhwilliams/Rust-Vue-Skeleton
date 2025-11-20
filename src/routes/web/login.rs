use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};

use crate::{app::AppState, extractors::Session, routes::WebError};

#[tracing::instrument(skip(app_state, session))]
pub async fn login(
    State(app_state): State<AppState>,
    mut session: Session,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, WebError> {
    if let Some(invite_code) = query.get("invite") {
        session.set("invite", invite_code).await?;
    }

    let (url, token, verifier) = app_state.oauth.get_oauth_url()?;

    session.set("csrf_token", token.into_secret()).await?;
    session.set("verifier", verifier).await?;

    Ok(Redirect::to(&url))
}
