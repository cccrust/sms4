use crate::model::shop_message::{self, ShopMessage, ShopMessageWithUser};
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SendBody {
    sender_id: i64,
    receiver_id: i64,
    shop_id: i64,
    content: String,
}

pub async fn send(
    State(state): State<AppState>,
    Json(body): Json<SendBody>,
) -> Result<Json<ShopMessage>, AppError> {
    let conn = state.conn.lock().unwrap();
    let msg = shop_message::send(&conn, body.shop_id, body.sender_id, body.receiver_id, &body.content)?;
    Ok(Json(msg))
}

#[derive(Deserialize)]
pub struct ListQuery {
    user_id: i64,
    other_id: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Path(shop_id): Path<i64>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<ShopMessageWithUser>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let msgs = shop_message::list_for_shop(&conn, shop_id, query.user_id, query.other_id)?;
    Ok(Json(msgs))
}

#[derive(Deserialize)]
pub struct ConvQuery {
    user_id: i64,
}

pub async fn conversations(
    State(state): State<AppState>,
    Query(query): Query<ConvQuery>,
) -> Result<Json<Vec<shop_message::ShopConversation>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let convs = shop_message::list_conversations(&conn, query.user_id)?;
    Ok(Json(convs))
}
