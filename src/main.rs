use rocket::response::status;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> status::Accepted<String> {
    status::Accepted(Some(String::from("Hello, world!")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
