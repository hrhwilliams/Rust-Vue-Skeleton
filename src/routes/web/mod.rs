mod errors;
mod index;
mod login;
mod logout;
mod redirect;

pub use errors::*;

use axum::{Router, routing::get};

use crate::app::AppState;

pub struct WebRoutes;

impl WebRoutes {
    pub fn router() -> Router<AppState> {
        Router::<AppState>::new()
            .route("/", get(index::index))
            .route("/oauth/redirect", get(redirect::redirect))
            .route("/oauth/finalize", get(redirect::finalize))
            .route("/admin/login", get(login::login))
            .route("/admin/logout", get(logout::logout))
        // .route("/admin/redirect", get(redirect))
    }
}
