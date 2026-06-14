use crate::model::shop::{self, Shop};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OpenShopBody {
    user_id: i64,
    name: String,
    description: Option<String>,
}

pub async fn open(
    State(state): State<AppState>,
    Json(body): Json<OpenShopBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match shop::open_shop(&conn, body.user_id, &body.name, body.description.as_deref()) {
        Ok(s) => Ok(Json(json!({ "shop": s }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

pub async fn my_shop(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<GetByUser>,
) -> Result<Json<Shop>, AppError> {
    let conn = state.conn.lock().unwrap();
    let s = shop::get_shop_by_user(&conn, params.user_id)?
        .ok_or_else(|| AppError::NotFound("該使用者沒有商店".into()))?;
    Ok(Json(s))
}

#[derive(Deserialize)]
pub struct GetByUser {
    user_id: i64,
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Shop>, AppError> {
    let conn = state.conn.lock().unwrap();
    let s = shop::get_shop_by_id(&conn, id)?
        .ok_or_else(|| AppError::NotFound("商店不存在".into()))?;
    Ok(Json(s))
}

#[derive(Deserialize)]
pub struct UpdateShopBody {
    user_id: i64,
    name: Option<String>,
    description: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateShopBody>,
) -> Result<Json<Shop>, AppError> {
    let conn = state.conn.lock().unwrap();
    let s = shop::get_shop_by_id(&conn, id)?
        .ok_or_else(|| AppError::NotFound("商店不存在".into()))?;
    if s.user_id != body.user_id {
        return Err(AppError::BadRequest("這不是你的商店".into()));
    }
    shop::update_shop(&conn, body.user_id, body.name.as_deref(), body.description.as_deref())?;
    let updated = shop::get_shop_by_id(&conn, id)?.unwrap();
    Ok(Json(updated))
}

#[derive(Deserialize)]
pub struct CloseShopBody {
    user_id: i64,
}

pub async fn close(
    State(state): State<AppState>,
    Json(body): Json<CloseShopBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if !shop::close_shop(&conn, body.user_id)? {
        return Err(AppError::NotFound("該使用者沒有商店".into()));
    }
    Ok(Json(json!({ "closed": true })))
}
