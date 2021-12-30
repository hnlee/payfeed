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

use clap::{App, Arg};
use futures::stream::Stream;
use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;

use crate::example_utils::setup_logger;

mod example_utils;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
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

pub async fn run_consumer() {
    let matches = App::new("consumer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line consumer")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .short("g")
                .long("group-id")
                .help("Consumer group id")
                .takes_value(true)
                .default_value("example_consumer_group_id"),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("topics")
                .short("t")
                .long("topics")
                .help("Topic list")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .get_matches();

    setup_logger(true, matches.value_of("log-conf"));

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    let brokers = matches.value_of("brokers").unwrap();
    let group_id = matches.value_of("group-id").unwrap();

    consume_and_print(brokers, group_id, &topics).await
}

// use rdkafka::consumer::stream_consumer::StreamConsumer;
// use rdkafka::consumer::Consumer;
// use rdkafka::Message;
// use std::boxed::Box;
// use tokio::runtime::current_thread::Runtime;

// mod utils;

// fn echo_message<M: Message>(msg: M) -> Result<(), std::str::Utf8Error> {
//     let deserialize = |o| match o {
//         None => Ok(""),
//         Some(val) => Ok(std::str::from_utf8(val)?),
//     };

//     println!(
//         "Consumed record from topic {} partition [{}] @ offset {} with key {} and value {}",
//         msg.topic(),
//         msg.partition(),
//         msg.offset(),
//         deserialize(msg.key())?,
//         deserialize(msg.payload())?,
//     );

//     Ok(())
// }

// fn run_consumer() -> Result<(), Box<dyn std::error::Error>> {
//     let (topic, mut config) = utils::get_config()?;
//     let consumer: StreamConsumer = config.set("group.id", "rust_example_group_1").create()?;

//     consumer.subscribe(&vec![topic.as_ref()])?;

//     let processor = consumer
//         .stream()
//         .filter_map(|result| match result {
//             Ok(_) => result.ok(),
//             Err(err) => {
//                 eprintln!("error consuming from message stream: {}", err);
//                 None
//             }
//         })
//         .for_each(|msg| echo_message(msg).map_err(|_| eprintln!("error deserializing message")));

//     Runtime::new()?
//         .block_on(processor)
//         .map_err(|_| eprintln!("error running consumer on current thread"))
//         .ok();

//     Ok(())
// }
