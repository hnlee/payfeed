use anyhow::Result;
use rust_decimal::Decimal;
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
    pub amount: Decimal,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Payment {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub amount: Decimal,
}

impl Payment {
    pub async fn create(new_payment: NewPayment, pool: &PgPool) -> Result<Payment> {
        let mut transaction = pool.begin().await?;
        let record: Payment = sqlx::query_as(
            "
            INSERT INTO payments (from_user, to_user, amount)
            VALUES ($1, $2, $3)
            RETURNING id, from_user, to_user, amount
            ",
        )
        .bind(new_payment.from_user)
        .bind(new_payment.to_user)
        .bind(new_payment.amount)
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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTransfer {
    pub for_user: Uuid,
    pub amount: Decimal,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Transfer {
    pub id: Uuid,
    pub for_user: Uuid,
    pub amount: Decimal,
}

impl Transfer {
    pub async fn create(new_transfer: NewTransfer, pool: &PgPool) -> Result<Transfer> {
        let mut transaction = pool.begin().await?;
        let record: Transfer = sqlx::query_as(
            "
            INSERT INTO transfers (for_user, amount)
            VALUES ($1, $2)
            RETURNING id, for_user, amount
            ",
        )
        .bind(new_transfer.for_user)
        .bind(new_transfer.amount)
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(Transfer {
            id: record.id,
            for_user: record.for_user,
            amount: record.amount,
        })
    }
}

use futures::stream::Stream;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use std::boxed::Box;
use tokio::runtime::current_thread::Runtime;

mod utils;

fn echo_message<M: Message>(msg: M) -> Result<(), std::str::Utf8Error> {
    let deserialize = |o| match o {
        None => Ok(""),
        Some(val) => Ok(std::str::from_utf8(val)?),
    };

    println!(
        "Consumed record from topic {} partition [{}] @ offset {} with key {} and value {}",
        msg.topic(),
        msg.partition(),
        msg.offset(),
        deserialize(msg.key())?,
        deserialize(msg.payload())?,
    );

    Ok(())
}

fn run_consumer() -> Result<(), Box<dyn std::error::Error>> {
    let (topic, mut config) = utils::get_config()?;
    let consumer: StreamConsumer = config.set("group.id", "rust_example_group_1").create()?;

    consumer.subscribe(&vec![topic.as_ref()])?;

    let processor = consumer
        .start()
        .filter_map(|result| match result {
            Ok(_) => result.ok(),
            Err(err) => {
                eprintln!("error consuming from message stream: {}", err);
                None
            }
        })
        .for_each(|msg| echo_message(msg).map_err(|_| eprintln!("error deserializing message")));

    Runtime::new()?
        .block_on(processor)
        .map_err(|_| eprintln!("error running consumer on current thread"))
        .ok();

    Ok(())
}
