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

#[derive(Clone, Default)]
struct AppState {
    counter: Arc<RwLock<u32>>,
}

impl AppState {
    pub async fn read(&self) -> u32 {
        *self.counter.read().await
    }

    pub async fn inc(&self) {
        let mut counter = self.counter.write().await;
        *counter += 1;
    }
}

#[derive(Serialize)]
struct ReadCounter {
    counter: u32,
}

async fn read_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    Json(ReadCounter {
        counter: app_state.read().await,
    })
}

async fn inc_counter(State(app_state): State<AppState>) -> impl IntoResponse {
    tracing::info!("Incremented counter state");

    app_state.inc().await;
    Json(ReadCounter {
        counter: app_state.read().await,
    })
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

    let router = axum::Router::new()
        .route("/counter", get(read_counter))
        .route("/counter", post(inc_counter))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    axum::serve(listener, router.into_make_service()).await
}
