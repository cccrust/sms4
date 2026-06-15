use crate::model::group::{self, Group, GroupMemberBrief, GroupWithOwner};
use crate::model::group_post::{self, GroupPostWithUser};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreateBody {
    user_id: i64,
    name: String,
    description: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> Result<Json<Group>, AppError> {
    let conn = state.conn.lock().unwrap();
    let g = group::create(&conn, body.user_id, &body.name, body.description.as_deref())?;
    Ok(Json(g))
}

#[derive(Deserialize)]
pub struct ListQuery {
    search: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<GroupWithOwner>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let groups = group::list(&conn, query.search.as_deref())?;
    Ok(Json(groups))
}

#[derive(Deserialize)]
pub struct MineQuery {
    user_id: i64,
}

pub async fn mine(
    State(state): State<AppState>,
    Query(query): Query<MineQuery>,
) -> Result<Json<Vec<GroupWithOwner>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let groups = group::list_my(&conn, query.user_id)?;
    Ok(Json(groups))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Group>, AppError> {
    let conn = state.conn.lock().unwrap();
    let g = group::get(&conn, id)?
        .ok_or_else(|| AppError::NotFound("社團不存在".into()))?;
    Ok(Json(g))
}

#[derive(Deserialize)]
pub struct UpdateBody {
    user_id: i64,
    name: Option<String>,
    description: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateBody>,
) -> Result<Json<Group>, AppError> {
    let conn = state.conn.lock().unwrap();
    group::update(&conn, id, body.user_id, body.name.as_deref(), body.description.as_deref())?;
    let g = group::get(&conn, id)?
        .ok_or_else(|| AppError::NotFound("社團不存在".into()))?;
    Ok(Json(g))
}

#[derive(Deserialize)]
pub struct DeleteBody {
    user_id: i64,
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<DeleteBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if group::delete(&conn, id, body.user_id)? {
        Ok(Json(json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound("社團不存在".into()))
    }
}

#[derive(Deserialize)]
pub struct JoinBody {
    user_id: i64,
}

pub async fn join(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<JoinBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match group::join(&conn, id, body.user_id) {
        Ok(true) => Ok(Json(json!({ "joined": true }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
        _ => unreachable!(),
    }
}

pub async fn leave(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<JoinBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match group::leave(&conn, id, body.user_id) {
        Ok(true) => Ok(Json(json!({ "left": true }))),
        Ok(false) => Err(AppError::NotFound("你不在這個社團中".into())),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

pub async fn members(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<GroupMemberBrief>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let members = group::list_members(&conn, id)?;
    Ok(Json(members))
}

#[derive(Deserialize)]
pub struct AddPostBody {
    user_id: i64,
    content: String,
}

pub async fn add_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<AddPostBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match group_post::add(&conn, id, body.user_id, &body.content) {
        Ok(p) => Ok(Json(json!({ "post": p }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

pub async fn list_posts(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<GroupPostWithUser>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let posts = group_post::list(&conn, id)?;
    Ok(Json(posts))
}

#[derive(Deserialize)]
pub struct DeletePostBody {
    user_id: i64,
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path((_group_id, post_id)): Path<(i64, i64)>,
    Json(body): Json<DeletePostBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if group_post::delete(&conn, post_id, body.user_id)? {
        Ok(Json(json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound("貼文不存在".into()))
    }
}
