mod me;

use axum::{Router, routing::get};

use crate::app::AppState;

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn router() -> Router<AppState> {
        Router::<AppState>::new().route("/auth/me", get(me::me))
    }
}
