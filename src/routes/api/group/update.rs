use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel}, errors::ApiError,
};

pub async fn update_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state
        .db
        .update_group(*id, create_group)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}
