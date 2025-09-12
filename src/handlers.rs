use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use crate::app::{set_env_var, AppState};
use crate::auth::{get_authenticated_user, Claims};
use crate::models::{CreateTodoDto, NewTodo, NewUser, Todo, UpdateTodoDto, User};
use crate::schema::users::dsl::users;
use diesel::{AsChangeset, QueryDsl};
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::{ExpressionMethods, OptionalExtension, QueryResult};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::dsl::exists;
use serde_json::json;
use uuid::Uuid;
use crate::schema::todos::dsl::todos;
use crate::schema::todos::{public_id, user_id};

pub async fn save_new_user(
    pool: &mut PgConnection,
    new_user: NewUser,
) -> diesel::result::QueryResult<User> {
    diesel::insert_into(crate::schema::users::table)
        .values(new_user)
        .returning(User::as_select())
        .get_result(pool)
}

pub async fn get_user_or_throw(pool: &mut PgConnection, alias: String) -> QueryResult<User> {
    QueryDsl::select(
        users.filter(crate::schema::users::alias.eq(alias)),
        User::as_select(),
    )
    .first::<User>(pool)
}

pub fn destructure_to_user(user: QueryResult<User>) -> User {
    user.unwrap_or(User{
        alias: "".to_string(),
        password: "".to_string(),
        created_at: None,
        id: 0
    })
}

pub async fn create_todo(
    headers: &HeaderMap,
    State(mut state): State<AppState>,
    Json(payload): Json<CreateTodoDto>,
) -> Result<Json<Todo>, (axum::http::StatusCode, String)> {
    let mut conn = &mut state.db_pool;

    let user: User = get_authenticated_user(&headers, &mut conn)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let mut pub_id = Uuid::new_v4();
    while diesel::select(diesel::dsl::exists(todos.filter(public_id.eq(pub_id))))
        .get_result::<bool>(conn)
        .unwrap_or(false)
    {
        pub_id = Uuid::new_v4();
    }


    let new_todo = NewTodo {
        title: payload.title,
        description: payload.description,
        public_id: pub_id,
        user_id: user.id
    };

    let inserted: Todo = diesel::insert_into(todos)
        .values(&new_todo)
        .returning(Todo::as_select())
        .get_result(conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inserted))

}

pub async fn update_todo(
    headers: axum::http::HeaderMap,
    State(mut state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodoDto>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    
    let mut conn = &mut state.db_pool;
    let user = get_authenticated_user(&headers, &mut conn)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    #[derive(AsChangeset)]
    #[diesel(table_name = crate::schema::todos)]
    struct TodoChangeset {
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    }

    let changes = TodoChangeset {
        title: payload.title,
        description: payload.description,
        completed: payload.completed,
    };

    let updated: Todo = diesel::update(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
        .set(changes)
        .returning(Todo::as_select())
        .get_result(conn)
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(updated))
}

pub async fn find_todo_by_id(
    headers: HeaderMap,
    State(mut state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = &mut state.db_pool;
    let user = get_authenticated_user(&headers, &mut conn)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let todo = QueryDsl::select(todos
        .filter(public_id.eq(todo_id))
        .filter(user_id.eq(user.id)), Todo::as_select())
        .first::<Todo>(conn)
        .map_err(|_| (StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(todo))
}

pub async fn find_todos_by_user(
    headers: HeaderMap,
    State(mut state): State<AppState>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let mut conn = &mut state.db_pool;
    let user = get_authenticated_user(&headers, &mut conn)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let results = QueryDsl::select(todos
        .filter(user_id.eq(user.id)), Todo::as_select())
        .load::<Todo>(conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}

pub async fn delete_todo(
    headers: axum::http::HeaderMap,
    State(mut state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    let mut conn = &mut state.db_pool;
    let user = get_authenticated_user(&headers, &mut conn)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let deleted_count = diesel::delete(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
        .execute(conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted_count == 0 {
        Err((StatusCode::NOT_FOUND, "Todo not found".to_string()))
    } else {
        Ok((StatusCode::OK, Json(json!({"status": "deleted"}))))
    }
}