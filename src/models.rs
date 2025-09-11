use diesel::{Insertable, Queryable, Selectable};
use crate::schema::{ users, todos };

#[derive( Debug,Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    alias: String,
    // created_at:
}


#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias: String
}

