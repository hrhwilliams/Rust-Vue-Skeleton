use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::{
    app::AppState,
    database::{Group, GroupModel},
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn get_all_groups(
    State(app_state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let groups: Vec<Group> = if query.is_empty() {
        app_state.db.get_all_groups().await
    } else {
        app_state.db.query_groups(query).await
    }?;

    Ok(Json(groups))
}

#[tracing::instrument(skip(app_state))]
pub async fn view_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = path.get("id").ok_or(ApiError::BadRequest)?;

    let group: Group = app_state.db.get_group(id).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(group))
}
