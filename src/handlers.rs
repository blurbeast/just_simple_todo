use diesel::{ExpressionMethods, OptionalExtension, QueryResult};
use diesel::QueryDsl;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel::query_dsl::select_dsl::SelectDsl;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;

pub async fn save_new_user(pool: &mut PgConnection, new_user: NewUser) -> diesel::result::QueryResult<User> {
    diesel::insert_into(crate::schema::users::table)
        .values(new_user)
        .returning(User::as_select())
        .get_result(pool)
}

pub async fn get_user_or_throw(pool: &mut PgConnection, alias: String) -> QueryResult<User> {
    QueryDsl::select(users
         .filter(crate::schema::users::alias.eq(alias)), User::as_select())
        .first::<User>(pool)
}

