use crate::model::auth;
use crate::model::user;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    password: String,
    display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LogoutPayload {
    token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if payload.username.trim().is_empty() || payload.password.len() < 4 {
        return Err(AppError::BadRequest("密碼長度至少需 4 個字元".into()));
    }
    if auth::is_username_taken(&conn, &payload.username)? {
        return Err(AppError::BadRequest("帳號已被使用".into()));
    }
    let display_name = payload.display_name.unwrap_or_else(|| payload.username.clone());
    let id = auth::register(&conn, &payload.username, &payload.password, &display_name)?;
    let u = user::get_user(&conn, id)?
        .ok_or_else(|| AppError::Internal("註冊後查詢使用者失敗".into()))?;
    Ok(Json(json!({ "user": u })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let token = auth::login(&conn, &payload.username, &payload.password)?;
    let user_id = auth::get_user_by_token(&conn, &token)?
        .ok_or_else(|| AppError::Internal("登入後取得使用者失敗".into()))?;
    let u = user::get_user(&conn, user_id)?
        .ok_or_else(|| AppError::Internal("查詢使用者失敗".into()))?;
    Ok(Json(json!({ "token": token, "user": u })))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if auth::logout(&conn, &payload.token)? {
        Ok(Json(json!({ "message": "已登出" })))
    } else {
        Err(AppError::BadRequest("Token 無效".into()))
    }
}
