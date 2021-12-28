#[macro_use]
extern crate diesel;

use diesel::PgConnection;
use diesel::RunQueryDsl;

use schema::users;
use schema::users::dsl::*;

pub mod schema;

#[rocket_sync_db_pools::database("postgres")]
pub struct PostgresConn(PgConnection);

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

pub fn add_user(new_user: NewUser, conn: &PgConnection) {
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .unwrap();
}
