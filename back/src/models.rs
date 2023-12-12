use crate::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserForm<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
    pub topology: serde_json::Value,
    pub updated_at: chrono::NaiveDateTime,
}
