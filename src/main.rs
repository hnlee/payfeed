use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use log::error;
use payfeed::{
    simple_consumer::consume_and_print, NewPayment, NewTransfer, NewUser, Payment, Transfer, User,
};
use sqlx::postgres::PgPool;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    //     let matches = App::new("consumer example")
    //         .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
    //         .about("Simple command line consumer")
    //         .arg(
    //             Arg::with_name("brokers")
    //                 .short("b")
    //                 .long("brokers")
    //                 .help("Broker list in kafka format")
    //                 .takes_value(true)
    //                 .default_value("localhost:9092"),
    //         )
    //         .arg(
    //             Arg::with_name("group-id")
    //                 .short("g")
    //                 .long("group-id")
    //                 .help("Consumer group id")
    //                 .takes_value(true)
    //                 .default_value("example_consumer_group_id"),
    //         )
    //         .arg(
    //             Arg::with_name("log-conf")
    //                 .long("log-conf")
    //                 .help("Configure the logging format (example: 'rdkafka=trace')")
    //                 .takes_value(true),
    //         )
    //         .arg(
    //             Arg::with_name("topics")
    //                 .short("t")
    //                 .long("topics")
    //                 .help("Topic list")
    //                 .takes_value(true)
    //                 .multiple(true)
    //                 .required(true),
    //         )
    //         .get_matches();

    //     setup_logger(true, matches.value_of("log-conf"));

    //     let (version_n, version_s) = get_rdkafka_version();
    //     info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    // consume_and_print(brokers, group_id, &topics).await;

    HttpResponse::Ok().body("Hello world!")
}

#[post("/users")]
async fn create_user(new_user: web::Json<NewUser>, pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(new_user.into_inner(), pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => {
            error!("Failed to create user: {}", err);
            HttpResponse::BadRequest().json("Failed to create user")
        }
    }
}

#[post("/payments")]
async fn create_payment(
    new_payment: web::Json<NewPayment>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Payment::create(new_payment.into_inner(), pool.get_ref()).await;
    match result {
        Ok(payment) => HttpResponse::Created().json(payment),
        Err(err) => {
            error!("Failed to create payment: {}", err);
            HttpResponse::BadRequest().json("Failed to create payment")
        }
    }
}

#[post("/transfers")]
async fn create_transfer(
    new_transfer: web::Json<NewTransfer>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Transfer::create(new_transfer.into_inner(), pool.get_ref()).await;
    match result {
        Ok(transfer) => HttpResponse::Created().json(transfer),
        Err(err) => {
            error!("Failed to create transfer: {}", err);
            HttpResponse::BadRequest().json("Failed to create transfer")
        }
    }
}

fn configure() -> (String, String, u16) {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("postgres://postgres:postgres@localhost:5432/postgres"));
    let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse::<u16>()
        .expect("PORT should be a u16");
    (database_url, host, port)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    let (database_url, host, port) = configure();

    let pool = PgPool::connect(&database_url[..]).await?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(create_user)
            .service(create_payment)
            .service(create_transfer)
    })
    .bind((&host[..], port))?
    .run()
    .await?;

    Ok(())
}
