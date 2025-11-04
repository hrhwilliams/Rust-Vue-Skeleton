use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel},
};

pub async fn update_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .update_group(*id, create_group)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
