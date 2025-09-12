use diesel::{Insertable, Queryable, Selectable};
use crate::schema::{ users, todos };
use chrono::{DateTime, Utc};

#[derive( Debug,Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub alias: String,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>
}


#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias: String
}

