mod errors;
mod model;

use std::sync::{Arc, atomic::AtomicBool};

pub use errors::*;
pub use model::*;

use sqlx::PgPool;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: PgPool,
    event_cache: Arc<RwLock<Arc<[Event]>>>,
    dirty: Arc<AtomicBool>,
}

impl PostgresDatabase {
    #[must_use]
    pub async fn new(postgres_url: &str) -> Self {
        let pool = PgPool::connect(postgres_url)
            .await
            .expect("Failed to connect to postgres");

        Self {
            pool,
            event_cache: Arc::default(),
            dirty: Arc::new(AtomicBool::new(true)),
        }
    }
}
