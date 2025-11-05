mod create;
mod delete;
mod update;
mod view;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::app::AppState;

pub struct GroupRoutes;

impl GroupRoutes {
    pub fn router() -> Router<AppState> {
        Router::<AppState>::new()
            // /groups to view all, /groups?name=... to query by name
            .route("/groups", get(view::get_all_groups))
            .route("/group/{id}", get(view::view_group))
            .route("/group/{id}", post(create::insert_group))
            .route("/group/{id}", put(update::update_group))
            .route("/group/{id}", delete(delete::delete_group))
    }
}
