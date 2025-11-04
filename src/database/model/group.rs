use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::PostgresDatabase;

#[derive(Serialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateGroup {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreatedGroup {
    pub group_id: Uuid,
}

#[async_trait]
pub trait GroupModel {
    async fn get_all_groups(&self) -> Result<Vec<Group>, sqlx::Error>;
    async fn get_group(&self, id: Uuid) -> Result<Option<Group>, sqlx::Error>;
    async fn insert_group(&self, create_group: CreateGroup) -> Result<CreatedGroup, sqlx::Error>;
    async fn update_group(&self, id: Uuid, create_group: CreateGroup) -> Result<(), sqlx::Error>;
    async fn delete_group(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl GroupModel for PostgresDatabase {
    async fn get_all_groups(&self) -> Result<Vec<Group>, sqlx::Error> {
        let groups = sqlx::query_as!(Group, "SELECT * FROM groups")
            .fetch_all(&self.pool)
            .await?;

        Ok(groups)
    }

    async fn get_group(&self, id: Uuid) -> Result<Option<Group>, sqlx::Error> {
        let group = sqlx::query_as!(Group, "SELECT * FROM groups WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(group)
    }

    async fn insert_group(&self, create_group: CreateGroup) -> Result<CreatedGroup, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO groups (id, name) VALUES ($1, $2)",
            id,
            create_group.name
        )
        .execute(&self.pool)
        .await?;

        Ok(CreatedGroup { group_id: id })
    }

    async fn update_group(&self, id: Uuid, create_group: CreateGroup) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE groups SET name = $2 WHERE id = $1",
            id,
            create_group.name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_group(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM groups WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
