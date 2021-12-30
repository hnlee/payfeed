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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPayment {
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub amount: f64,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Payment {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub amount: f64,
}

impl Payment {
    pub async fn create(new_payment: NewPayment, pool: &PgPool) -> Result<User> {
        let mut transaction = pool.begin().await?;
        let record: Payment = sqlx::query_as(
            "
            INSERT INTO payments (from_user, to_user, amount)
            VALUES ($1, $2, $3)
            RETURNING id, from_user, to_user, amount
            ",
        )
        .bind(new_payment.name)
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(Payment {
            id: record.id,
            from_user: record.from_user,
            to_user: record.to_user,
            amount: record.amount,
        })
    }
}
