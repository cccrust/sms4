use crate::model::block;
use crate::model::user;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct BlockPayload {
    blocker_id: i64,
    blocked_id: i64,
}

pub async fn block(
    State(state): State<AppState>,
    Json(payload): Json<BlockPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    user::get_user(&conn, payload.blocker_id)?
        .ok_or_else(|| AppError::NotFound("封鎖者不存在".into()))?;
    user::get_user(&conn, payload.blocked_id)?
        .ok_or_else(|| AppError::NotFound("被封鎖者不存在".into()))?;
    block::block_user(&conn, payload.blocker_id, payload.blocked_id)?;
    Ok(Json(json!({ "message": "已封鎖" })))
}

pub async fn unblock(
    State(state): State<AppState>,
    Json(payload): Json<BlockPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if block::unblock_user(&conn, payload.blocker_id, payload.blocked_id)? {
        Ok(Json(json!({ "message": "已解除封鎖" })))
    } else {
        Err(AppError::NotFound("封鎖記錄不存在".into()))
    }
}

pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<user::UserBrief>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let users = block::list_blocked_users(&conn, user_id)?;
    Ok(Json(users))
}

pub async fn check(
    State(state): State<AppState>,
    Path((user_id, other_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let blocked = block::is_blocked(&conn, user_id, other_id)?;
    Ok(Json(json!({ "blocked": blocked })))
}
