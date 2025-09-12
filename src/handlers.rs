use crate::app::set_env_var;
use crate::auth::Claims;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;
use diesel::QueryDsl;
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::{ExpressionMethods, OptionalExtension, QueryResult};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

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
    })
}

