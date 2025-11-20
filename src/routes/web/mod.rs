mod errors;
mod index;
mod login;
mod redirect;

pub use errors::*;

use axum::{Router, routing::get};

use crate::app::AppState;

pub struct WebRoutes;

impl WebRoutes {
    pub fn router() -> Router<AppState> {
        Router::<AppState>::new()
            .route("/", get(index::index))
            .route("/redirect", get(redirect::redirect))
            .route("/admin/login", get(login::login))
        // .route("/admin/redirect", get(redirect))
    }
}
