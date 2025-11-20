use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app::AppState,
    database::{CreateGroup, GroupModel},
    extractors::AuthenticatedApiUser,
    routes::ApiError,
};

#[tracing::instrument(skip(app_state))]
pub async fn insert_group(
    AuthenticatedApiUser(_user_agent): AuthenticatedApiUser,
    State(app_state): State<AppState>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, ApiError> {
    let created_group = app_state.db.insert_group(create_group).await?;

    Ok(Json(created_group))
}
