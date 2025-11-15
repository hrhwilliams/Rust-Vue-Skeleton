use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::app::AppState;

mod create;
mod delete;
mod update;
mod view;

pub struct EventRoutes;

impl EventRoutes {
    pub fn router() -> Router<AppState> {
        Router::<AppState>::new()
            // /events to view all, /events?group_id=... to query by group_id
            .route("/events", get(view::get_all_events))
            .route("/event/{id}", get(view::view_event))
            .route("/event", post(create::insert_event))
            .route("/event/{id}", put(update::update_event))
            .route("/event/{id}", delete(delete::delete_event))
    }
}
