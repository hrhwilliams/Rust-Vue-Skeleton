mod model;

pub use model::*;

use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(postgres_url: &str) -> Self {
        let pool = PgPool::connect(postgres_url)
            .await
            .expect("Failed to connect to postgres");

        Self { pool }
    }
}
