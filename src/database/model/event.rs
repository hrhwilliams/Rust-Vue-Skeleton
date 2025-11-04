use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::PostgresDatabase;

#[derive(Serialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub group_id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateEvent {
    pub group_id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
}

#[derive(Serialize)]
pub struct CreatedEvent {
    pub event_id: Uuid,
}

#[async_trait]
pub trait EventModel {
    async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error>;
    async fn get_event(&self, id: Uuid) -> Result<Option<Event>, sqlx::Error>;
    async fn insert_event(&self, create_event: CreateEvent) -> Result<CreatedEvent, sqlx::Error>;
    async fn update_event(&self, id: Uuid, create_event: CreateEvent) -> Result<(), sqlx::Error>;
    async fn delete_event(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl EventModel for PostgresDatabase {
    async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error> {
        let events = sqlx::query_as!(Event, "SELECT * FROM events")
            .fetch_all(&self.pool)
            .await?;

        Ok(events)
    }

    async fn get_event(&self, id: Uuid) -> Result<Option<Event>, sqlx::Error> {
        let event = sqlx::query_as!(Event, "SELECT * FROM events WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(event)
    }

    async fn insert_event(&self, create_event: CreateEvent) -> Result<CreatedEvent, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query!(
            r#"INSERT INTO events
              (id, group_id, name, description, starts_at, ends_at)
            VALUES
              ($1, $2, $3, $4, $5, $6)"#,
            id,
            create_event.group_id,
            create_event.name,
            create_event.description,
            create_event.starts_at,
            create_event.ends_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(CreatedEvent { event_id: id })
    }

    async fn update_event(&self, id: Uuid, create_event: CreateEvent) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE events SET
              group_id = $2, name = $3, description = $4, starts_at = $5, ends_at = $6
            WHERE
              id = $1"#,
            id,
            create_event.group_id,
            create_event.name,
            create_event.description,
            create_event.starts_at,
            create_event.ends_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_event(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM events WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
