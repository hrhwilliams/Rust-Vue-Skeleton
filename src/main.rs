use std::env;

use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use rust_vue_skeleton::{
    app::AppState,
    database::PostgresDatabase,
    routes::{EventRoutes, GroupRoutes},
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

/// A struct that gets serialized to a JSON and returned as a response to a GET
/// request to "/counter"
#[derive(Serialize)]
struct ReadCounter {
    counter: u32,
}

/// Endpoint to return the current state of the counter as a JSON
async fn read_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    Json(ReadCounter {
        counter: app_state.read(),
    })
}

/// Endpoint to increment the counter. Returns nothing
async fn inc_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state.inc();
}

/// Passthrough to the Vue app
async fn vue_passthrough() -> impl IntoResponse {
    Html(include_str!("../frontend/dist/index.html"))
}

fn get_db_name() -> String {
    format!(
        "postgres://{}:{}@localhost:{}/{}",
        std::env::var("DB_USER").expect("DB_USER not set"),
        std::env::var("DB_PASS").expect("DB_PASS not set"),
        std::env::var("DB_PORT").expect("DB_PORT not set"),
        std::env::var("DB_NAME").expect("DB_NAME not set"),
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port: u16 = env::var("APP_PORT")
        .expect("APP_PORT not set")
        .parse()
        .expect("APP_PORT malformed");

    let postgres_url = env::var("DATABASE_URL").unwrap_or(get_db_name());
    let db = PostgresDatabase::new(&postgres_url).await;

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Failed to bind to address");

    let app_state = AppState::new(db);

    let files = ServeDir::new("./frontend/dist");

    // Define routes for GET to "/counter" and POST to "/counter", bundle app
    // state, and allow cross origin requests (will be coming from the frontend)
    let router = axum::Router::new()
        .route("/api/counter", get(read_counter))
        .route("/api/counter", post(inc_counter))
        .nest("/api", EventRoutes::router())
        .nest("/api", GroupRoutes::router())
        .route("/", get(vue_passthrough))
        .layer(TraceLayer::new_for_http())
        .fallback_service(files)
        .with_state(app_state);

    axum::serve(listener, router.into_make_service()).await
}
