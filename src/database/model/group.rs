use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, prelude::FromRow};
use time::OffsetDateTime;

use crate::database::PostgresDatabase;

#[derive(Serialize, FromRow)]
pub struct Group {
    pub vrc_group_id: String,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub vrc_group_id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreatedGroup {
    pub group_id: String,
}

#[async_trait]
pub trait GroupModel {
    async fn get_all_groups(&self) -> Result<Vec<Group>, sqlx::Error>;
    async fn query_groups(&self, query: HashMap<String, String>) -> Result<Vec<Group>, sqlx::Error>;
    async fn get_group(&self, id: &str) -> Result<Option<Group>, sqlx::Error>;
    async fn insert_group(&self, create_group: CreateGroup) -> Result<CreatedGroup, sqlx::Error>;
    async fn update_group(&self, id: &str, create_group: CreateGroup) -> Result<(), sqlx::Error>;
    async fn delete_group(&self, id: &str) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl GroupModel for PostgresDatabase {
    async fn get_all_groups(&self) -> Result<Vec<Group>, sqlx::Error> {
        let groups = sqlx::query_as!(Group, "SELECT * FROM groups")
            .fetch_all(&self.pool)
            .await?;

        Ok(groups)
    }

    async fn query_groups(&self, query: HashMap<String, String>) -> Result<Vec<Group>, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM groups WHERE 1=1");

        if let Some(name) = query.get("name") {
            query_builder.push(" AND name = ");
            query_builder.push_bind(name);
        }

        let query = query_builder.build_query_as::<Group>();
        Ok(query.fetch_all(&self.pool).await?)
    }

    async fn get_group(&self, id: &str) -> Result<Option<Group>, sqlx::Error> {
        let group = sqlx::query_as!(Group, "SELECT * FROM groups WHERE vrc_group_id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(group)
    }

    async fn insert_group(&self, create_group: CreateGroup) -> Result<CreatedGroup, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO groups (vrc_group_id, name) VALUES ($1, $2)",
            create_group.vrc_group_id,
            create_group.name
        )
        .execute(&self.pool)
        .await?;

        Ok(CreatedGroup {
            group_id: create_group.vrc_group_id,
        })
    }

    async fn update_group(&self, id: &str, create_group: CreateGroup) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE groups SET name = $2 WHERE vrc_group_id = $1",
            id,
            create_group.name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_group(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM groups WHERE vrc_group_id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
