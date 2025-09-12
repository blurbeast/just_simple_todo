use crate::app::{AppState, set_env_var};
use crate::handlers::{destructure_to_user, get_user_or_throw, save_new_user};
use crate::models::{NewUser, User};
use crate::schema::users::alias;
use crate::schema::users::dsl::users;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: i64,
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    alias: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}
pub async fn register(
    mut state: State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if payload.alias.trim().is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "username and password required"})),
        );
    }

    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to hash password"})),
            );
        }
    };
    let new_user = NewUser::new(payload.alias, password_hash);
    let _ = save_new_user(&mut state.db_pool, new_user);

    (StatusCode::CREATED, Json(json!({"status": "registered"})))
}

pub async fn login(
    mut state: State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // let alias = payload.alias.clone();
    match get_user_or_throw(&mut state.db_pool, payload.alias).await {
        Ok(user) => match verify(&payload.password, &user.password) {
            Ok(true) => {
                let expiration = (OffsetDateTime::now_utc() + Duration::hours(1)).unix_timestamp();
                let claims = Claims {
                    sub: user.alias,
                    exp: expiration,
                };
                let JWT_SECRET: String = set_env_var("JWT_SECRET");
                let header = Header::default();
                match encode(
                    &header,
                    &claims,
                    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
                ) {
                    Ok(token) => (StatusCode::OK, Json(json!({ "token": token }))),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": format!("token error: {}", e) })),
                    ),
                }
            }
            _ => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid username or password"})),
            ),
        },
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "User not found"
            })),
        ),
    }
}

pub async fn get_authenticated_user(headers: &axum::http::HeaderMap, pool: &mut PgConnection) -> Result<User, String> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or("Missing or invalid Authorization header")?;

    let JWT_SECRET: String = set_env_var("JWT_SECRET");

    let validation = jsonwebtoken::Validation::default();

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )
    .map_err(|e| format!("Token invalid: {}", e))?;

    let user = destructure_to_user(get_user_or_throw(pool, token_data.claims.sub).await);

    Ok(user)
}
