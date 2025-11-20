use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    app::AppState,
    database::{Event, EventModel},
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn get_all_events(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let events = if query.is_empty() {
        app_state.db.get_all_events().await
    } else {
        app_state.db.query_events(query).await
    }?;

    Ok(Json(events))
}

#[tracing::instrument(skip(app_state))]
pub async fn view_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    let event: Event = app_state.db.get_event(id).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(event))
}
