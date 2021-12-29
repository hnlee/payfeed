use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub async fn create(new_user: NewUser, pool: &PgPool) -> Result<User> {
        let mut transaction = pool.begin().await?;
        let record: User = sqlx::query_as(
            "
            INSERT INTO users (name)
            VALUES ($1)
            RETURNING id, name
            ",
        )
        .bind(new_user.name)
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(User {
            id: record.id,
            name: record.name,
        })
    }
}
