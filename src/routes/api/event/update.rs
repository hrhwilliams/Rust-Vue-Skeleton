use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
    errors::ApiError,
};

pub async fn update_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state
        .db
        .update_event(*id, create_event)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}
