use std::collections::HashMap;

use async_trait::async_trait;
use base64::{Engine, prelude::BASE64_URL_SAFE};
use rand::{TryRngCore, rngs::OsRng};
use serde::Serialize;
use serde_json::Value;
use sqlx::prelude::FromRow;
use time::{Duration, OffsetDateTime};

use crate::database::{DatabaseError, PostgresDatabase};

#[derive(Serialize, FromRow)]
struct Session {
    id: String,
    #[serde(with = "time::serde::rfc3339")]
    expires: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    store: String,
}

#[async_trait]
pub trait SessionModel {
    async fn new_session(&self) -> Result<String, DatabaseError>;
    async fn get_session_store(&self, session: &str) -> Result<Option<HashMap<String, Value>>, DatabaseError>;
    async fn update_session_store(&self, session: &str, store: &HashMap<String, Value>) -> Result<(), DatabaseError>;
    async fn delete_expired_sessions(&self) -> Result<(), DatabaseError>;
}

#[async_trait]
impl SessionModel for PostgresDatabase {
    async fn new_session(&self) -> Result<String, DatabaseError> {
        let mut bytes = [0u8; 33];
        OsRng.try_fill_bytes(&mut bytes).map_err(|_| DatabaseError::RngError)?;
        let session = BASE64_URL_SAFE.encode(bytes);

        let expires = OffsetDateTime::now_utc().checked_add(Duration::days(7));

        sqlx::query!("INSERT INTO sessions (id, expires) VALUES ($1, $2)", session, expires)
            .execute(&self.pool)
            .await?;
        Ok(session)
    }

    async fn get_session_store(&self, session: &str) -> Result<Option<HashMap<String, Value>>, DatabaseError> {
        if let Some(session) = sqlx::query_as!(Session, "SELECT * FROM sessions WHERE id = $1", session)
            .fetch_optional(&self.pool)
            .await?
        {
            let json: HashMap<String, Value> =
                serde_json::from_str(&session.store).map_err(DatabaseError::SerdeError)?;
            Ok(Some(json))
        } else {
            Ok(None)
        }
    }

    async fn update_session_store(&self, session: &str, store: &HashMap<String, Value>) -> Result<(), DatabaseError> {
        sqlx::query!(
            "UPDATE sessions SET store = $1 WHERE id = $2",
            serde_json::to_string(store).map_err(|e| DatabaseError::SerdeError(e))?,
            session
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_expired_sessions(&self) -> Result<(), DatabaseError> {
        sqlx::query!("DELETE FROM sessions WHERE expires > $1", OffsetDateTime::now_utc())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
