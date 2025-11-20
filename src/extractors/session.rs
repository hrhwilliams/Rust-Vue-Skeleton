use std::collections::HashMap;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::app::AppState;
use crate::database::DatabaseError;
use crate::database::PostgresDatabase;
use crate::database::SessionModel;
use crate::extractors::SessionError;

#[derive(Clone)]
pub struct Session {
    session: String,
    store: HashMap<String, Value>,
    db: PostgresDatabase,
}

impl Session {
    pub fn new(session: &str, store: HashMap<String, Value>, db: PostgresDatabase) -> Self {
        Self {
            session: session.to_string(),
            store,
            db,
        }
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        if let Some(value) = self.store.get(key) {
            Ok(Some(serde_json::from_value(value.clone())?))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T>(&mut self, key: &str, value: T) -> Result<(), DatabaseError>
    where
        T: Serialize,
    {
        self.store.insert(
            key.to_string(),
            serde_json::to_value(value).map_err(DatabaseError::SerdeError)?,
        );
        self.db.update_session_store(&self.session, &self.store).await
    }

    pub async fn remove(&mut self, key: &str) -> Result<(), DatabaseError> {
        self.store.remove(key);
        self.db.update_session_store(&self.session, &self.store).await
    }
}

impl FromRequestParts<AppState> for Session {
    type Rejection = SessionError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| SessionError::ExtractError)?;

        let session_id = jar
            .get("__Host-Http-Session")
            .ok_or_else(|| SessionError::NoSession)?
            .value();

        let session_store = state
            .db
            .get_session_store(session_id)
            .await
            .map_err(SessionError::DatabaseError)?
            .ok_or_else(|| SessionError::NoSession)?;

        let session = Session::new(session_id, session_store, state.db.clone());

        Ok(session)
    }
}
