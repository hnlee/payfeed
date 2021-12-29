use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    let (database_url, host, port) = configure();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url[..])
        .await?;

    HttpServer::new(move || App::new().data(pool.clone()).service(hello))
        .bind((&host[..], port))?
        .run()
        .await?;

    Ok(())
}
