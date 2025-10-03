use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDateTime;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewObject, Object};

pub fn create_object(conn: &mut PgConnection, d: &NaiveDateTime, t: &String, p: &f32, s: &f32) -> Object {
    use crate::schema::objects;

    let new_object = NewObject { d, t, p, s };

    diesel::insert_into(objects::table)
        .values(&new_object)
        .returning(Object::as_returning())
        .get_result(conn)
        .expect("Error saving new object")
}
