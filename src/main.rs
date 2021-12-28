#[macro_use]
extern crate rocket;

use payfeed::{NewUser, PostgresConn};
use rocket::response::status;

#[get("/")]
fn index() -> status::Accepted<String> {
    status::Accepted(Some(String::from("Hello, world!")))
}

#[post("/users")]
async fn post_users(pool: PostgresConn) -> status::Accepted<String> {
    let new_user = NewUser {
        name: "placeholder",
    };
    pool.run(|conn| payfeed::add_user(new_user, conn)).await;
    status::Accepted(Some(String::from("User created")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresConn::fairing())
        .mount("/", routes![index])
        .mount("/users", routes![post_users])
}
