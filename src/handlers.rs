// use crate::app::AppState;
use crate::auth::get_authenticated_user;
use crate::models::{ClientUser, CreateTodoDto, NewTodo, NewUser, Todo, UpdateTodoDto, User};
use crate::schema::todos::dsl::todos;
use crate::schema::todos::{public_id, user_id};
use crate::schema::users::dsl::users;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::{AsChangeset, QueryDsl};
use diesel::ExpressionMethods;
use diesel::{RunQueryDsl, SelectableHelper};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
// pub async fn save_new_user(
//     State(state): State<Arc<AppState>>,
//     new_user: NewUser,
// ) ->  Result<Json<User>, (StatusCode, String)> {
//
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let h = diesel::insert_into(crate::schema::users::table)
//         .values(new_user)
//         .returning(User::as_select())
//         .get_result(&mut conn).map_err(|e| {
//         if e.to_string().contains("unique constraint") {
//             (
//                 StatusCode::CONFLICT,
//                 "Note with this title already exists".to_string(),
//             )
//         } else {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Database error: {}", e),
//             )
//         }
//     })?;
//
//     Ok(Json(h))
//
// }
//
// pub async fn get_user_or_throw(
//     State(state): State<Arc<AppState>>,
//     alias: String,
// ) -> Result<User, (StatusCode, String)> {
//         let mut conn = state.db_pool.get().map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("DB connection error: {}", e),
//             )
//         })?;
//         users
//             .filter(crate::schema::users::alias.eq(alias))
//             .select(User::as_select())
//             .first::<User>(&mut conn)
//             .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))
// }
//
// pub fn destructure_to_user(user: QueryResult<User>) -> User {
//     user.unwrap_or(User{
//         alias: "".to_string(),
//         password: "".to_string(),
//         created_at: None,
//         id: 0
//     })
// }
//
// pub async fn create_todo(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<CreateTodoDto>,
// ) -> Result<Json<Todo>, (StatusCode, String)> {
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let user: User = get_authenticated_user(&headers, State(state))
//         .await
//         .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
//
//     let mut pub_id = Uuid::new_v4();
//     while diesel::select(diesel::dsl::exists(todos.filter(public_id.eq(pub_id))))
//         .get_result::<bool>(&mut conn)
//         .unwrap_or(false)
//     {
//         pub_id = Uuid::new_v4();
//     }
//
//     let new_todo = NewTodo {
//         title: payload.title,
//         description: payload.description,
//         public_id: pub_id,
//         user_id: user.id,
//     };
//
//     let inserted: Todo = diesel::insert_into(todos)
//         .values(&new_todo)
//         .returning(Todo::as_select())
//         .get_result(&mut conn)
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
//
//     Ok(Json(inserted))
// }
//
// pub async fn update_todo(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
//     Path(todo_id): Path<Uuid>,
//     Json(payload): Json<UpdateTodoDto>,
// ) -> Result<Json<Todo>, (StatusCode, String)> {
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let user = get_authenticated_user(&headers, State(state))
//         .await
//         .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
//
//     #[derive(AsChangeset)]
//     #[diesel(table_name = crate::schema::todos)]
//     struct TodoChangeset {
//         title: Option<String>,
//         description: Option<String>,
//         completed: Option<bool>,
//     }
//
//     let changes = TodoChangeset {
//         title: payload.title,
//         description: payload.description,
//         completed: payload.completed,
//     };
//
//     let updated: Todo = diesel::update(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
//         .set(changes)
//         .returning(Todo::as_select())
//         .get_result(&mut conn)
//         .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
//
//     Ok(Json(updated))
// }
// pub async fn find_todo_by_id(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
//     Path(todo_id): Path<Uuid>,
// ) -> Result<Json<Todo>, (StatusCode, String)> {
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let user = get_authenticated_user(&headers, State(state))
//         .await
//         .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
//
//     let todo = todos
//         .filter(public_id.eq(todo_id))
//         .filter(user_id.eq(user.id))
//         .select(Todo::as_select())
//         .first::<Todo>(&mut conn)
//         .map_err(|_| (StatusCode::NOT_FOUND, "Todo not found".to_string()))?;
//
//     Ok(Json(todo))
// }
//
//
// pub async fn find_todos_by_user(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
// ) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let user = get_authenticated_user(&headers, State(state))
//         .await
//         .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
//
//     let results = todos
//         .filter(user_id.eq(user.id))
//         .select(Todo::as_select())
//         .load::<Todo>(&mut conn)
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
//
//     Ok(Json(results))
// }
//
// pub async fn delete_todo(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
//     Path(todo_id): Path<Uuid>,
// ) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
//     let mut conn = state.db_pool.get().map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Failed to get DB connection: {}", e),
//         )
//     })?;
//
//     let user = get_authenticated_user(&headers, State(state))
//         .await
//         .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
//
//     let deleted_count = diesel::delete(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
//         .execute(&mut conn)
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
//
//     if deleted_count == 0 {
//         Err((StatusCode::NOT_FOUND, "Todo not found".to_string()))
//     } else {
//         Ok((StatusCode::OK, Json(json!({ "status": "deleted" }))))
//     }
// }

pub async fn save_new_user(
    State(state): State<Arc<AppState>>,
    new_user: NewUser,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let result = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_select())
        .get_result::<User>(&mut conn);

    match result {
        Ok(user) => (
            StatusCode::CREATED,
            Json(json!({ "status": "registered", "user": ClientUser{
                alias: user.alias,
                created_at: user.created_at
            }  }))
        ),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "User with this alias already exists" })),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Database error: {}", e) })),
                )
            }
        }
    }
}

pub async fn get_user_or_throw(
    State(state): State<Arc<AppState>>,
    alias: String,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("DB connection error: {}", e) })),
            );
        }
    };

    match users
        .filter(crate::schema::users::alias.eq(alias))
        .select(User::as_select())
        .first::<User>(&mut conn)
    {
        Ok(user) => (StatusCode::OK, Json(json!({ "user": user }))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
    }
}

pub async fn create_todo(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoDto>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let user = match get_authenticated_user(&headers, State(state.clone())).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": e })),
            );
        }
    };

    let mut pub_id = Uuid::new_v4();
    while diesel::select(diesel::dsl::exists(todos.filter(public_id.eq(pub_id))))
        .get_result::<bool>(&mut conn)
                .unwrap_or(false)
    {
        pub_id = Uuid::new_v4();
    }

    let new_todo = NewTodo {
        title: payload.title,
        description: payload.description,
        public_id: pub_id,
        user_id: user.id,
    };

    match diesel::insert_into(todos)
        .values(&new_todo)
        .returning(Todo::as_select())
        .get_result::<Todo>(&mut conn)

    {
        Ok(todo) => (StatusCode::CREATED, Json(json!({ "todo": todo }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        ),
    }
}

pub async fn update_todo(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodoDto>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let user = match get_authenticated_user(&headers, State(state.clone())).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": e })),
            );
        }
    };

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

    match diesel::update(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
        .set(changes)
        .returning(Todo::as_select())
        .get_result::<Todo>(&mut conn)

    {
        Ok(todo) => (StatusCode::OK, Json(json!({ "todo": todo }))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Todo not found" })),
        ),
    }
}

pub async fn find_todo_by_id(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let user = match get_authenticated_user(&headers, State(state.clone())).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": e })),
            );
        }
    };

    match todos
        .filter(public_id.eq(todo_id))
        .filter(user_id.eq(user.id))
        .select(Todo::as_select())
        .first::<Todo>(&mut conn)

    {
        Ok(todo) => (StatusCode::OK, Json(json!({ "todo": todo }))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Todo not found" })),
        ),
    }
}

pub async fn find_todos_by_user(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let user = match get_authenticated_user(&headers, State(state.clone())).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": e })),
            );
        }
    };

    match todos
        .filter(user_id.eq(user.id))
        .select(Todo::as_select())
        .load::<Todo>(&mut conn)

    {
        Ok(todo) => (StatusCode::OK, Json(json!({ "todos": todo }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        ),
    }
}

pub async fn delete_todo(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Path(todo_id): Path<Uuid>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut conn = match state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to get DB connection: {}", e) })),
            );
        }
    };

    let user = match get_authenticated_user(&headers, State(state.clone())).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": e })),
            );
        }
    };

    match diesel::delete(todos.filter(public_id.eq(todo_id)).filter(user_id.eq(user.id)))
        .execute(&mut conn)
    {
        Ok(0) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Todo not found" })),
        ),
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "status": "deleted" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        ),
    }
}