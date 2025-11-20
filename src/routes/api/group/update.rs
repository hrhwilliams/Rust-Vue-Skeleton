use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel},
    extractors::AuthenticatedApiUser,
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn update_group(
    AuthenticatedApiUser(_user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, String>>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state.db.update_group(id, create_group).await?;

    Ok(())
}
