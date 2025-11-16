use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, prelude::FromRow};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::database::PostgresDatabase;

#[derive(Serialize, FromRow)]
pub struct Event {
    pub vrc_event_id: String,
    pub vrc_group_id: String,
    pub name: String,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
    pub category: String,
    pub access_type: String,
    pub platforms: Vec<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub vrc_event_id: String,
    pub vrc_group_id: String,
    pub name: String,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
    pub category: String,
    pub access_type: String,
    pub platforms: Vec<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct CreatedEvent {
    pub vrc_event_id: String,
}

#[async_trait]
pub trait EventModel {
    async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error>;
    async fn query_events(&self, query: HashMap<String, String>) -> Result<Vec<Event>, sqlx::Error>;
    async fn get_event(&self, id: &str) -> Result<Option<Event>, sqlx::Error>;
    async fn insert_event(&self, create_event: CreateEvent) -> Result<CreatedEvent, sqlx::Error>;
    async fn update_event(&self, id: &str, create_event: CreateEvent) -> Result<(), sqlx::Error>;
    async fn delete_event(&self, id: &str) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl EventModel for PostgresDatabase {
    async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error> {
        let events = sqlx::query_as!(Event, "SELECT * FROM events")
            .fetch_all(&self.pool)
            .await?;

        Ok(events)
    }

    async fn query_events(&self, query: HashMap<String, String>) -> Result<Vec<Event>, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM events WHERE 1=1");

        if let Some(starts_at_str) = query.get("starts_at") {
            let starts_at = OffsetDateTime::parse(starts_at_str, &Rfc3339)
                .map_err(|e| sqlx::Error::InvalidArgument(e.to_string()))?;
            // .with_timezone(&Utc);
            query_builder.push(" AND starts_at >= ");
            query_builder.push_bind(starts_at);
        }

        if let Some(ends_at_str) = query.get("ends_at") {
            let ends_at = OffsetDateTime::parse(ends_at_str, &Rfc3339)
                .map_err(|e| sqlx::Error::InvalidArgument(e.to_string()))?;
            query_builder.push(" AND ends_at <= ");
            query_builder.push_bind(ends_at);
        }

        if let Some(group_id) = query.get("group_id") {
            query_builder.push(" AND vrc_group_id = ");
            query_builder.push_bind(group_id);
        }

        let query = query_builder.build_query_as::<Event>();
        Ok(query.fetch_all(&self.pool).await?)
    }

    async fn get_event(&self, id: &str) -> Result<Option<Event>, sqlx::Error> {
        let event = sqlx::query_as!(Event, "SELECT * FROM events WHERE vrc_event_id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(event)
    }

    async fn insert_event(&self, create_event: CreateEvent) -> Result<CreatedEvent, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO events
              (vrc_event_id, vrc_group_id, name, description, starts_at, ends_at, category, access_type, platforms, image_url, tags)
            VALUES
              ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
            create_event.vrc_event_id,
            create_event.vrc_group_id,
            create_event.name,
            create_event.description,
            create_event.starts_at,
            create_event.ends_at,
            create_event.category,
            create_event.access_type,
            &create_event.platforms,
            create_event.image_url,
            create_event.tags.as_deref(),
        )
        .execute(&self.pool)
        .await?;

        Ok(CreatedEvent {
            vrc_event_id: create_event.vrc_event_id,
        })
    }

    async fn update_event(&self, id: &str, create_event: CreateEvent) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE events SET
              name = $2, description = $3, starts_at = $4, ends_at = $5, category = $6, access_type = $7, platforms = $8, image_url = $9, tags = $10
            WHERE
              vrc_event_id = $1"#,
            id,
            create_event.name,
            create_event.description,
            create_event.starts_at,
            create_event.ends_at,
            create_event.category,
            create_event.access_type,
            &create_event.platforms,
            create_event.image_url,
            create_event.tags.as_deref(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_event(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM events WHERE vrc_event_id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
