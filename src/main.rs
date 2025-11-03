use std::{
    env,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering::Relaxed},
    },
};

use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

/// Stores the value of the counter. This is passed around to the endpoints
/// and modified by them.
#[derive(Clone)]
struct AppState {
    db: PgPool,
    counter: Arc<AtomicU32>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            counter: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Return the value of the counter
    pub fn read(&self) -> u32 {
        self.counter.load(Relaxed)
    }

    /// Increment the counter
    pub fn inc(&self) {
        self.counter.fetch_add(1, Relaxed);
    }
}

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
    let db = PgPool::connect(&postgres_url)
        .await
        .expect("Failed to connect to postgres");

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
        .route("/", get(vue_passthrough))
        .layer(TraceLayer::new_for_http())
        .fallback_service(files)
        .with_state(app_state);

    axum::serve(listener, router.into_make_service()).await
}
