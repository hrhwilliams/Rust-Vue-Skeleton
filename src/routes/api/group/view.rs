use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app::AppState, database::{GroupModel, Group}, errors::ApiError};

pub async fn get_all_groups(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let groups: Vec<Group> = if query.is_empty() {
        app_state
            .db
            .get_all_groups()
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    } else {
        app_state
            .db
            .query_groups(query)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }?;

    Ok(Json(groups))
}

pub async fn view_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    let group: Group = app_state
        .db
        .get_group(*id)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(group))
}
