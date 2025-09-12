use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::app::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: i64
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    alias: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String
}
pub fn register
(
    state: State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<serde_json::Value>)
{
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


}

pub fn login() {}