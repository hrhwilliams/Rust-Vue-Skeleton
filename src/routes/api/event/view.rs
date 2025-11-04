use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app::AppState, database::EventModel};

pub async fn get_all_events(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let events = app_state
        .db
        .get_all_events()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}

pub async fn view_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    let event = app_state
        .db
        .get_event(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(event))
}
