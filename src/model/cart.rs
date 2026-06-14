use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CartItem {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CartItemWithDetails {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub price: i64,
    pub stock: i64,
    pub quantity: i64,
    pub total_price: i64,
    pub shop_id: i64,
    pub shop_name: String,
    pub created_at: String,
}

pub fn add(conn: &Connection, user_id: i64, product_id: i64, quantity: i64) -> Result<CartItem> {
    if quantity <= 0 {
        bail!("數量必須大於 0");
    }
    let product = crate::model::product::get_product(conn, product_id)?
        .ok_or_else(|| anyhow::anyhow!("商品不存在"))?;
    if product.stock < quantity {
        bail!("庫存不足（剩 {}）", product.stock);
    }
    conn.execute(
        "INSERT INTO cart_items (user_id, product_id, quantity) VALUES (?1, ?2, ?3)
         ON CONFLICT(user_id, product_id) DO UPDATE SET quantity = quantity + ?3",
        params![user_id, product_id, quantity],
    )?;
    let id = conn.last_insert_rowid();
    if id == 0 {
        let mut stmt = conn.prepare("SELECT id, user_id, product_id, quantity, created_at FROM cart_items WHERE user_id = ?1 AND product_id = ?2")?;
        let item = stmt.query_row(params![user_id, product_id], |row| {
            Ok(CartItem {
                id: row.get(0)?,
                user_id: row.get(1)?,
                product_id: row.get(2)?,
                quantity: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        return Ok(item);
    }
    get(conn, id)?.ok_or_else(|| anyhow::anyhow!("加入購物車失敗"))
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<CartItem>> {
    let mut stmt = conn.prepare("SELECT id, user_id, product_id, quantity, created_at FROM cart_items WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(CartItem {
            id: row.get(0)?,
            user_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    match rows.next() {
        Some(Ok(c)) => Ok(Some(c)),
        _ => Ok(None),
    }
}

pub fn list(conn: &Connection, user_id: i64) -> Result<Vec<CartItemWithDetails>> {
    let mut stmt = conn.prepare(
        "SELECT ci.id, ci.user_id, ci.product_id, p.name, p.price, p.stock, ci.quantity,
                p.price * ci.quantity, p.shop_id, s.name, ci.created_at
         FROM cart_items ci
         JOIN products p ON p.id = ci.product_id
         JOIN shops s ON s.id = p.shop_id
         WHERE ci.user_id = ?1
         ORDER BY ci.created_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(CartItemWithDetails {
            id: row.get(0)?,
            user_id: row.get(1)?,
            product_id: row.get(2)?,
            product_name: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            quantity: row.get(6)?,
            total_price: row.get(7)?,
            shop_id: row.get(8)?,
            shop_name: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn remove(conn: &Connection, id: i64, user_id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM cart_items WHERE id = ?1 AND user_id = ?2", params![id, user_id])?;
    Ok(n > 0)
}

pub fn update_quantity(conn: &Connection, id: i64, user_id: i64, quantity: i64) -> Result<bool> {
    if quantity <= 0 {
        return remove(conn, id, user_id);
    }
    let n = conn.execute(
        "UPDATE cart_items SET quantity = ?1 WHERE id = ?2 AND user_id = ?3",
        params![quantity, id, user_id],
    )?;
    Ok(n > 0)
}

pub fn checkout(conn: &Connection, user_id: i64) -> Result<Vec<i64>> {
    let items = list(conn, user_id)?;
    if items.is_empty() {
        bail!("購物車是空的");
    }
    let mut order_ids = Vec::new();
    for item in &items {
        let product = crate::model::product::get_product(conn, item.product_id)?
            .ok_or_else(|| anyhow::anyhow!("商品 {} 已不存在", item.product_id))?;
        if product.stock < item.quantity {
            bail!("商品「{}」庫存不足（剩 {}）", product.name, product.stock);
        }
        let total_price = product.price * item.quantity;
        conn.execute(
            "INSERT INTO orders (buyer_id, product_id, quantity, total_price) VALUES (?1, ?2, ?3, ?4)",
            params![user_id, item.product_id, item.quantity, total_price],
        )?;
        let order_id = conn.last_insert_rowid();
        crate::model::product::decrement_stock(conn, item.product_id, item.quantity)?;
        order_ids.push(order_id);
    }
    conn.execute("DELETE FROM cart_items WHERE user_id = ?1", params![user_id])?;
    Ok(order_ids)
}

pub fn count(conn: &Connection, user_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COALESCE(SUM(quantity), 0) FROM cart_items WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )?;
    Ok(count)
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

    fn setup() -> (Connection, i64, i64) {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let s = shop::open_shop(&c, uid, "Shop", None).unwrap();
        let p = product::add_product(&c, s.id, "咖啡豆", 299, 10, None).unwrap();
        (c, uid, p.id)
    }

    #[test]
    fn test_add_and_list() {
        let (c, uid, pid) = setup();
        let item = add(&c, uid, pid, 2).unwrap();
        assert_eq!(item.quantity, 2);
        let items = list(&c, uid).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].total_price, 598);
    }

    #[test]
    fn test_add_duplicate_increases_quantity() {
        let (c, uid, pid) = setup();
        add(&c, uid, pid, 2).unwrap();
        add(&c, uid, pid, 3).unwrap();
        let items = list(&c, uid).unwrap();
        assert_eq!(items[0].quantity, 5);
    }

    #[test]
    fn test_checkout() {
        let (c, uid, pid) = setup();
        add(&c, uid, pid, 2).unwrap();
        let ids = checkout(&c, uid).unwrap();
        assert_eq!(ids.len(), 1);
        assert!(list(&c, uid).unwrap().is_empty());
        let p = product::get_product(&c, pid).unwrap().unwrap();
        assert_eq!(p.stock, 8);
    }

    #[test]
    fn test_empty_cart_checkout_fails() {
        let c = conn();
        let uid = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        assert!(checkout(&c, uid).is_err());
    }
}
