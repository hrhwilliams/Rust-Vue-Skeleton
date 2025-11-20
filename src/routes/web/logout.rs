use axum::response::{IntoResponse, Redirect};

use crate::{extractors::Session, routes::WebError};

#[tracing::instrument(skip(session))]
pub async fn logout(mut session: Session) -> Result<impl IntoResponse, WebError> {
    session.remove("token").await?;
    Ok(Redirect::to("/"))
}
