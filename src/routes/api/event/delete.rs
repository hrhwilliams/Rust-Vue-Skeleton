use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{app::AppState, database::EventModel, extractors::AuthenticatedApiUser, routes::ApiError};

#[tracing::instrument(skip(app_state))]
pub async fn delete_event(
    AuthenticatedApiUser(_user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state.db.delete_event(id).await?;

    Ok(())
}
