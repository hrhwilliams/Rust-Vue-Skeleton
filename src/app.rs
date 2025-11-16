use axum::{
    Router,
    body::Body,
    http::{HeaderName, Request},
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;

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
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        HeaderName::from_static("x-request-id"),
                        MakeRequestUuid,
                    ))
                    .layer(
                        TraceLayer::new_for_http()
                            .make_span_with(|request: &Request<Body>| {
                                let id = request
                                    .headers()
                                    .get("x-request-id")
                                    .and_then(|value| value.to_str().ok())
                                    .unwrap_or("unknown");
                                tracing::span!(
                                    Level::INFO,
                                    "request",
                                    id = id,
                                    method = %request.method(),
                                    uri = %request.uri(),
                                    user_agent = tracing::field::Empty,
                                )
                            })
                            .on_response(DefaultOnResponse::new().level(Level::INFO)),
                    ),
            )
            .fallback_service(files)
            .with_state(app_state);

        Self { router }
    }

    pub async fn serve(self, listener: TcpListener) -> Result<(), std::io::Error> {
        axum::serve(listener, self.router.into_make_service()).await
    }
}
