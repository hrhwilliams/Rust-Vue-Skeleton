use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel},
    errors::ApiError,
    extractors::AuthenticatedApiUser,
};

pub async fn insert_group(
    AuthenticatedApiUser(_user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, ApiError> {
    let created_group = app_state
        .db
        .insert_group(create_group)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(created_group))
}
