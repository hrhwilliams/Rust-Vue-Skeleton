use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    database::{Event, EventModel},
    errors::ApiError,
};

pub async fn get_all_events(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let events: Vec<Event> = if query.is_empty() {
        app_state
            .db
            .get_all_events()
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    } else {
        app_state
            .db
            .query_events(query)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }?;

    Ok(Json(events))
}

pub async fn view_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    let event: Event = app_state
        .db
        .get_event(*id)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(event))
}
