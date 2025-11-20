use axum::response::{IntoResponse, Redirect};

use crate::{extractors::WebSession, routes::WebError};

#[tracing::instrument(skip(session))]
pub async fn logout(WebSession(mut session): WebSession) -> Result<impl IntoResponse, WebError> {
    session.remove("token").await?;
    Ok(Redirect::to("/"))
}
