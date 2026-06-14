use crate::model::order::{self, Order, OrderWithDetails};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOrderBody {
    buyer_id: i64,
    product_id: i64,
    #[serde(default = "default_quantity")]
    quantity: i64,
}

fn default_quantity() -> i64 {
    1
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateOrderBody>,
) -> Result<Json<Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    match order::create_order(&conn, body.buyer_id, body.product_id, body.quantity) {
        Ok(o) => Ok(Json(o)),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct ListOrdersQuery {
    user_id: i64,
}

pub async fn list(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ListOrdersQuery>,
) -> Result<Json<Vec<OrderWithDetails>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let orders = order::list_orders(&conn, params.user_id)?;
    Ok(Json(orders))
}

#[derive(Deserialize)]
pub struct GetOrderQuery {
    user_id: i64,
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    axum::extract::Query(params): axum::extract::Query<GetOrderQuery>,
) -> Result<Json<Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    let o = order::get_order(&conn, id)?
        .ok_or_else(|| AppError::NotFound("訂單不存在".into()))?;
    if o.buyer_id != params.user_id {
        return Err(AppError::BadRequest("這不是你的訂單".into()));
    }
    Ok(Json(o))
}
