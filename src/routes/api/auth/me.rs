use axum::{Json, extract::State, response::IntoResponse};

use crate::{app::AppState, extractors::Session, routes::ApiError};

#[tracing::instrument(skip(app_state, session))]
pub(crate) async fn me(State(app_state): State<AppState>, session: Session) -> Result<impl IntoResponse, ApiError> {
    let token = session
        .get("token")
        .map_err(|_| ApiError::BadRequest)?
        .ok_or(ApiError::Unauthorized(None))?;
    let info = app_state.oauth.get_discord_info(&token).await?;

    Ok(Json(info))
}
