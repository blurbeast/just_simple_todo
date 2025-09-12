use crate::schema::{todos, users};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub alias: String,
    pub password: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub alias: String,
    pub password: String,
}

impl NewUser {
    pub fn new(alias: String, pass: String) -> Self {
        Self {
            alias,
            password: pass,
        }
    }
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    // pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub public_id: Uuid,
    pub completed: bool,
    pub updated_at: Option<String>,
}
#[derive(Debug, Insertable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    pub public_id: Uuid,
    pub user_id: i32
}

#[derive(Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
