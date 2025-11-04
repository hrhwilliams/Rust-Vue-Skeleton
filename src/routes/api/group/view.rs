use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app::AppState, database::GroupModel};

pub async fn get_all_groups(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let groups = app_state
        .db
        .get_all_groups()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(groups))
}

pub async fn view_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    let group = app_state
        .db
        .get_group(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(group))
}
