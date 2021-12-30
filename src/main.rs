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
    payfeed::run_consumer().await;
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
