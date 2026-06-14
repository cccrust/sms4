use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: i64,
    pub buyer_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub total_price: i64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderWithDetails {
    pub id: i64,
    pub buyer_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub shop_name: String,
    pub shop_user_id: i64,
    pub quantity: i64,
    pub total_price: i64,
    pub status: String,
    pub created_at: String,
}

pub fn create_order(conn: &Connection, buyer_id: i64, product_id: i64, quantity: i64) -> Result<Order> {
    if quantity <= 0 {
        bail!("數量必須大於 0");
    }
    let product = crate::model::product::get_product(conn, product_id)?
        .ok_or_else(|| anyhow::anyhow!("商品不存在"))?;
    if product.stock < quantity {
        bail!("庫存不足（剩 {}）", product.stock);
    }
    let total_price = product.price * quantity;
    conn.execute(
        "INSERT INTO orders (buyer_id, product_id, quantity, total_price) VALUES (?1, ?2, ?3, ?4)",
        params![buyer_id, product_id, quantity, total_price],
    )?;
    let id = conn.last_insert_rowid();
    crate::model::product::decrement_stock(conn, product_id, quantity)?;
    get_order(conn, id)?.ok_or_else(|| anyhow::anyhow!("建立訂單失敗"))
}

pub fn get_order(conn: &Connection, id: i64) -> Result<Option<Order>> {
    let mut stmt = conn.prepare("SELECT id, buyer_id, product_id, quantity, total_price, status, created_at, updated_at FROM orders WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Order {
            id: row.get(0)?,
            buyer_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
            total_price: row.get(4)?,
            status: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(o)) => Ok(Some(o)),
        _ => Ok(None),
    }
}

pub fn list_orders(conn: &Connection, buyer_id: i64) -> Result<Vec<OrderWithDetails>> {
    let mut stmt = conn.prepare(
        "SELECT o.id, o.buyer_id, o.product_id, p.name, s.name, s.user_id, o.quantity, o.total_price, o.status, o.created_at
         FROM orders o
         JOIN products p ON p.id = o.product_id
         JOIN shops s ON s.id = p.shop_id
         WHERE o.buyer_id = ?1
         ORDER BY o.created_at DESC",
    )?;
    let rows = stmt.query_map(params![buyer_id], |row| {
        Ok(OrderWithDetails {
            id: row.get(0)?,
            buyer_id: row.get(1)?,
            product_id: row.get(2)?,
            product_name: row.get(3)?,
            shop_name: row.get(4)?,
            shop_user_id: row.get(5)?,
            quantity: row.get(6)?,
            total_price: row.get(7)?,
            status: row.get(8)?,
            created_at: row.get(9)?,
        })
    })?;
    let mut orders = Vec::new();
    for row in rows {
        orders.push(row?);
    }
    Ok(orders)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::{product, shop, user};

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_create_and_list() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let s = shop::open_shop(&c, uid, "Shop", None).unwrap();
        let p = product::add_product(&c, s.id, "咖啡豆", 299, 10, None).unwrap();

        let buyer = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let order = create_order(&c, buyer, p.id, 2).unwrap();
        assert_eq!(order.total_price, 598);
        assert_eq!(order.status, "pending");

        let list = list_orders(&c, buyer).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].product_name, "咖啡豆");
    }

    #[test]
    fn test_insufficient_stock() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let s = shop::open_shop(&c, uid, "Shop", None).unwrap();
        let p = product::add_product(&c, s.id, "咖啡豆", 299, 3, None).unwrap();
        let buyer = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        assert!(create_order(&c, buyer, p.id, 10).is_err());
    }
}
