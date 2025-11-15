use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app::AppState, database::GroupModel, errors::ApiError};

pub async fn delete_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    app_state
        .db
        .delete_group(*id)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}
