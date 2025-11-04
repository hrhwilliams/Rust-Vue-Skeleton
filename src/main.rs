use std::{
    collections::HashMap,
    env,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering::Relaxed},
    },
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
};
use rust_vue_skeleton::database::{
    CreateEvent, CreateGroup, EventModel, GroupModel, PostgresDatabase,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use uuid::Uuid;

/// Stores the value of the counter. This is passed around to the endpoints
/// and modified by them.
#[derive(Clone)]
struct AppState {
    db: PostgresDatabase,
    counter: Arc<AtomicU32>,
}

impl AppState {
    pub fn new(db: PostgresDatabase) -> Self {
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

async fn get_all_events(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let events = app_state
        .db
        .get_all_events()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}

async fn insert_event(
    State(app_state): State<AppState>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, StatusCode> {
    let created_event = app_state
        .db
        .insert_event(create_event)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_event))
}

async fn view_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    let event = app_state
        .db
        .get_event(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(event))
}

async fn update_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_event): Json<CreateEvent>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .update_event(*id, create_event)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn delete_event(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .delete_event(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_all_groups(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let groups = app_state
        .db
        .get_all_groups()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(groups))
}

async fn insert_group(
    State(app_state): State<AppState>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, StatusCode> {
    let created_group = app_state
        .db
        .insert_group(create_group)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_group))
}

async fn view_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    let group = app_state
        .db
        .get_group(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(group))
}

async fn update_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
    Json(create_group): Json<CreateGroup>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .update_group(*id, create_group)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn delete_group(
    State(app_state): State<AppState>,
    Path(path): Path<HashMap<String, Uuid>>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = path.get("id").ok_or(StatusCode::BAD_REQUEST)?;

    app_state
        .db
        .delete_group(*id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
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
        .route("/api/events", get(get_all_events))
        .route("/api/event", post(insert_event))
        .route("/api/event/{id}", get(view_event))
        .route("/api/event/{id}", put(update_event))
        .route("/api/event/{id}", delete(delete_event))
        .route("/api/groups", get(get_all_groups))
        .route("/api/group", post(insert_group))
        .route("/api/group/{id}", get(view_group))
        .route("/api/group/{id}", put(update_group))
        .route("/api/group/{id}", delete(delete_group))
        .route("/", get(vue_passthrough))
        .layer(TraceLayer::new_for_http())
        .fallback_service(files)
        .with_state(app_state);

    axum::serve(listener, router.into_make_service()).await
}
