use crate::schema::users;
use diesel::prelude::*;
use rocket::time::PrimitiveDateTime;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserForm<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
    pub topology: serde_json::Value,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct ContainerStats {
    pub container_id: String,
    pub timestamp: PrimitiveDateTime,
    pub cpu_usage: f64,
    pub memory_usage: i32,
    pub io_usage_read: f64,
    pub io_usage_write: f64,
    pub network_usage_up: f64,
    pub network_usage_down: f64,
}
