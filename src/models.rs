use diesel::{Insertable, Queryable};
use crate::schema::rustaceans;

#[derive(Queryable)]
struct  Rustacean {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "rustaceans"]
struct NewRustacean {
    name: String,
    email: String,
}

