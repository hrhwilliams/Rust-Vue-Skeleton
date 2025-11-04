use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
};

pub async fn insert_event(
    State(app_state): State<AppState>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, StatusCode> {
    let created_event = app_state
        .db
        .insert_event(create_event)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_event))
}
