use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
    extractors::AuthenticatedApiUser,
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn update_event(
    AuthenticatedApiUser(_user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, String>>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state
        .db
        .update_event(id, create_event)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}
