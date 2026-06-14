use crate::model::cart::{self, CartItemWithDetails};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct AddBody {
    user_id: i64,
    product_id: i64,
    #[serde(default = "default_qty")]
    quantity: i64,
}

fn default_qty() -> i64 { 1 }

pub async fn add(
    State(state): State<AppState>,
    Json(body): Json<AddBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match cart::add(&conn, body.user_id, body.product_id, body.quantity) {
        Ok(item) => Ok(Json(json!({ "cart_item": item }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

pub async fn list(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ListQuery>,
) -> Result<Json<Vec<CartItemWithDetails>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = cart::list(&conn, params.user_id)?;
    Ok(Json(items))
}

#[derive(Deserialize)]
pub struct ListQuery {
    user_id: i64,
}

#[derive(Deserialize)]
pub struct RemoveBody {
    user_id: i64,
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<RemoveBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if cart::remove(&conn, id, body.user_id)? {
        Ok(Json(json!({ "removed": true })))
    } else {
        Err(AppError::NotFound("購物車項目不存在".into()))
    }
}

#[derive(Deserialize)]
pub struct UpdateQtyBody {
    user_id: i64,
    quantity: i64,
}

pub async fn update_quantity(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateQtyBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    cart::update_quantity(&conn, id, body.user_id, body.quantity)?;
    Ok(Json(json!({ "updated": true })))
}

#[derive(Deserialize)]
pub struct CheckoutBody {
    user_id: i64,
}

pub async fn checkout(
    State(state): State<AppState>,
    Json(body): Json<CheckoutBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match cart::checkout(&conn, body.user_id) {
        Ok(ids) => Ok(Json(json!({ "order_ids": ids }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct CountQuery {
    user_id: i64,
}

pub async fn count(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<CountQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let n = cart::count(&conn, params.user_id)?;
    Ok(Json(json!({ "count": n })))
}
