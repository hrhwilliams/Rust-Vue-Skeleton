use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel},
};

pub async fn insert_group(
    State(app_state): State<AppState>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, StatusCode> {
    let created_group = app_state
        .db
        .insert_group(create_group)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_group))
}
