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

use futures::StreamExt;
use log::{info, warn};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::{Headers, Message};
use rdkafka::util::get_rdkafka_version;

pub async fn consume_and_print(consumer: &StreamConsumer) {
    info!("I have called the consume_and_print function");
    let mut message_stream = consumer.start();

    info!("I have started the consumer");
    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

pub async fn setup_consumer() -> Result<StreamConsumer> {
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topics = vec![
        "app.public.users",
        "app.public.transfers",
        "app.public.payments",
    ];
    let brokers = "kafka:9092";
    let group_id = "app_consumer_group";

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    Ok(consumer)
}

use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::{LevelFilter, Record};
use std::io::Write;
use std::thread;

pub fn setup_logger(log_thread: bool, log_level: &str) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        write!(
            formatter,
            "{} {}{} - {} - {}\n",
            time_str,
            thread_name,
            record.level(),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info)
        .parse_filters(log_level);

    builder.init();
}
