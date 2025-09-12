use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::{users, todos };

#[derive( Debug,Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub alias: String,
    pub created_at: Option<String>
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias: String
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub public_id: Uuid,
    pub completed: bool,
    pub updated_at: Option<String>
}
#[derive(Debug, Insertable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    pub public_id: Uuid,
}
