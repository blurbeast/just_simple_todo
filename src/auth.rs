use crate::app::set_env_var;
use crate::handlers::{get_user_or_throw, save_new_user};
use crate::models::{NewUser, User};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub alias: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}


pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if payload.alias.trim().is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "username and password required" })),
        );
    }

    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "failed to hash password" })),
            );
        }
    };

    let new_user = NewUser {
        alias: payload.alias,
        password: password_hash,
    };
    save_new_user(State(state), new_user).await
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let (status, json) = get_user_or_throw(State(state.clone()), payload.alias.clone()).await;

    if status != StatusCode::OK {
        return (status, json);
    }

    let user = json["user"].clone();
    let user: User = serde_json::from_value(user).expect("Failed to deserialize user");

    match bcrypt::verify(&payload.password, &user.password) {
        Ok(true) => {
            let expiration = (OffsetDateTime::now_utc() + Duration::hours(1)).unix_timestamp();
            let claims = Claims {
                sub: user.alias,
                exp: expiration,
            };
            let jwt_secret = set_env_var("JWT_SECRET");
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_ref()),
            ) {
                Ok(token) => (
                    StatusCode::OK,
                    Json(json!({ "token": token })),
                ),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("token error: {}", e) })),
                ),
            }
        }
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "invalid username or password" })),
        ),
    }
}

pub async fn get_authenticated_user(
    headers: &HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<User, String> {
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or("Missing or invalid Authorization header")?;

    let jwt_secret = set_env_var("JWT_SECRET");
    let validation = Validation::default();

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
        .map_err(|e| format!("Token invalid: {}", e))?;

    let (status, json) = get_user_or_throw(State(state), token_data.claims.sub).await;
    if status != StatusCode::OK {
        return Err("User not found".to_string());
    }

    let user: User = serde_json::from_value(json["user"].clone())
        .expect("Failed to deserialize user");
    Ok(user)
}