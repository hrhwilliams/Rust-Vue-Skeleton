use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
};

pub async fn update_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .update_event(*id, create_event)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
