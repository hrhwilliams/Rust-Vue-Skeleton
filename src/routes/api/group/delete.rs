use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app::AppState, database::GroupModel};

pub async fn delete_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .delete_group(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
