use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    database::PostgresDatabase,
    routes::{EventRoutes, GroupRoutes},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PostgresDatabase,
}

impl AppState {
    pub fn new(db: PostgresDatabase) -> Self {
        Self { db }
    }
}

/// Passthrough to the Vue app
async fn vue_passthrough() -> impl IntoResponse {
    Html(include_str!("../frontend/dist/index.html"))
}

pub struct App {
    router: Router,
}

impl App {
    pub fn new(db: PostgresDatabase) -> Self {
        let app_state = AppState::new(db);
        let files = ServeDir::new("./frontend/dist");

        let router = Router::new()
            .nest("/api", EventRoutes::router())
            .nest("/api", GroupRoutes::router())
            .route("/", get(vue_passthrough))
            .layer(TraceLayer::new_for_http())
            .fallback_service(files)
            .with_state(app_state);

        Self { router }
    }

    pub async fn serve(self, listener: TcpListener) -> Result<(), std::io::Error> {
        axum::serve(listener, self.router.into_make_service()).await
    }
}
