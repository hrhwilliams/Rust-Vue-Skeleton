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
use crate::routes::ApiError;
use crate::routes::WebError;

#[derive(Clone)]
pub struct Session {
    session: String,
    store: HashMap<String, Value>,
    db: PostgresDatabase,
}

impl Session {
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

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, SessionError> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| SessionError::ExtractError)?;

        let session_id = jar
            .get("__Host-Http-Session")
            .ok_or_else(|| SessionError::NoSession)?
            .value()
            .to_string();

        tracing::info!(session_id, "querying for session ID");
        let session_store = state
            .db
            .get_session_store(&session_id)
            .await
            .map_err(SessionError::DatabaseError)?
            .ok_or_else(|| SessionError::NoSessionInDatabase)?;

        Ok(Self {
            session: session_id,
            store: session_store,
            db: state.db.clone(),
        })
    }
}

#[derive(Clone)]
pub struct WebSession(pub Session);

#[derive(Clone)]
pub struct ApiSession(pub Session);

impl FromRequestParts<AppState> for WebSession {
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;
        Ok(WebSession(session))
    }
}

impl FromRequestParts<AppState> for ApiSession {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;
        Ok(ApiSession(session))
    }
}
