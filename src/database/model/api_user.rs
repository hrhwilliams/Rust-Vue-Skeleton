use async_trait::async_trait;
use serde::Serialize;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use crate::database::{DatabaseError, PostgresDatabase};

#[derive(Serialize, FromRow)]
pub struct ApiUser {
    pub api_key: String,
    pub user_agent: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[async_trait]
pub trait ApiUserModel {
    async fn validate_api_key(&self, api_key: &str) -> Result<bool, DatabaseError>;
}

#[async_trait]
impl ApiUserModel for PostgresDatabase {
    async fn validate_api_key(&self, api_key: &str) -> Result<bool, DatabaseError> {
        let user_maybe = sqlx::query_as!(ApiUser, "SELECT * FROM api_users WHERE api_key = $1", api_key)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(user) = user_maybe {
            Ok(user.api_key == api_key)
        } else {
            Ok(false)
        }
    }
}
