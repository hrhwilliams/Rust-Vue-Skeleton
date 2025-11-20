use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateEvent, EventModel},
    extractors::AuthenticatedApiUser,
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn insert_event(
    AuthenticatedApiUser(user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, ApiError> {
    let created_event = app_state
        .db
        .insert_event(create_event)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(created_event))
}
