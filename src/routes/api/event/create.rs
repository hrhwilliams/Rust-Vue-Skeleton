use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
    errors::ApiError,
};

#[axum::debug_handler]
pub async fn insert_event(
    State(app_state): State<AppState>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, ApiError> {
    let created_event = app_state
        .db
        .insert_event(create_event)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(created_event))
}
