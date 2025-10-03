use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::objects;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = objects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Object {
    pub id: i32,
    pub d: NaiveDateTime,
    pub t: String,
    pub p: f32,
    pub s: f32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = objects)]
pub struct NewObject<'a> {
    pub d: &'a NaiveDateTime,
    pub t: &'a String,
    pub p: &'a f32,
    pub s: &'a f32,
}
