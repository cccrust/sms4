use crate::model::product::{self, Product, ProductWithShop};
use crate::model::shop;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct AddProductBody {
    user_id: i64,
    name: String,
    price: i64,
    #[serde(default)]
    stock: i64,
    description: Option<String>,
}

pub async fn add(
    State(state): State<AppState>,
    Path(shop_id): Path<i64>,
    Json(body): Json<AddProductBody>,
) -> Result<Json<Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let s = shop::get_shop_by_id(&conn, shop_id)?
        .ok_or_else(|| AppError::NotFound("商店不存在".into()))?;
    if s.user_id != body.user_id {
        return Err(AppError::BadRequest("這不是你的商店".into()));
    }
    let p = product::add_product(&conn, shop_id, &body.name, body.price, body.stock, body.description.as_deref())?;
    Ok(Json(p))
}

pub async fn list_by_shop(
    State(state): State<AppState>,
    Path(shop_id): Path<i64>,
) -> Result<Json<Vec<Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let products = product::list_products(&conn, shop_id)?;
    Ok(Json(products))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = product::get_product(&conn, id)?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;
    Ok(Json(p))
}

#[derive(Deserialize)]
pub struct UpdateProductBody {
    user_id: i64,
    name: Option<String>,
    price: Option<i64>,
    stock: Option<i64>,
    description: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateProductBody>,
) -> Result<Json<Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = product::get_product(&conn, id)?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;
    let s = shop::get_shop_by_id(&conn, p.shop_id)?
        .ok_or_else(|| AppError::NotFound("商店不存在".into()))?;
    if s.user_id != body.user_id {
        return Err(AppError::BadRequest("這不是你的商品".into()));
    }
    product::update_product(&conn, id, body.name.as_deref(), body.price, body.stock, body.description.as_deref())?;
    let updated = product::get_product(&conn, id)?.unwrap();
    Ok(Json(updated))
}

#[derive(Deserialize)]
pub struct RemoveProductBody {
    user_id: i64,
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<RemoveProductBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = product::get_product(&conn, id)?
        .ok_or_else(|| AppError::NotFound("商品不存在".into()))?;
    let s = shop::get_shop_by_id(&conn, p.shop_id)?
        .ok_or_else(|| AppError::NotFound("商店不存在".into()))?;
    if s.user_id != body.user_id {
        return Err(AppError::BadRequest("這不是你的商品".into()));
    }
    product::remove_product(&conn, id)?;
    Ok(Json(json!({ "removed": true })))
}

#[derive(Deserialize)]
pub struct SearchParams {
    q: Option<String>,
    min_price: Option<i64>,
    max_price: Option<i64>,
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<ProductWithShop>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let products = product::search_products(&conn, params.q.as_deref(), params.min_price, params.max_price)?;
    Ok(Json(products))
}
