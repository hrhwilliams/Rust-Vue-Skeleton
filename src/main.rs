use std::{env, sync::Arc};

use axum::{
    Json,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;

/// Stores the value of the counter. This is passed around to the endpoints
/// and modified by them.
#[derive(Clone, Default)]
struct AppState {
    counter: Arc<RwLock<u32>>,
}

impl AppState {
    /// Return the value of the counter
    pub async fn read(&self) -> u32 {
        *self.counter.read().await
    }

    /// Increment the counter
    pub async fn inc(&self) {
        let mut counter = self.counter.write().await;
        *counter += 1;
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
        counter: app_state.read().await,
    })
}

/// Endpoint to increment the counter. Returns nothing
async fn inc_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state.inc().await;
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .expect("APP_PORT not set")
        .parse()
        .expect("APP_PORT malformed");

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Failed to bind to address");

    let app_state = AppState::default();

    // Define routes for GET to "/counter" and POST to "/counter", bundle app
    // state, and allow cross origin requests (will be coming from the frontend)
    let router = axum::Router::new()
        .route("/counter", get(read_counter))
        .route("/counter", post(inc_counter))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    axum::serve(listener, router.into_make_service()).await
}
