use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Product {
    pub id: i64,
    pub shop_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub stock: i64,
    pub image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProductWithShop {
    pub id: i64,
    pub shop_id: i64,
    pub shop_name: String,
    pub shop_user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub stock: i64,
    pub image: Option<String>,
    pub created_at: String,
}

pub fn add_product(conn: &Connection, shop_id: i64, name: &str, price: i64, stock: i64, description: Option<&str>) -> Result<Product> {
    conn.execute(
        "INSERT INTO products (shop_id, name, description, price, stock) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![shop_id, name, description, price, stock],
    )?;
    let id = conn.last_insert_rowid();
    get_product(conn, id)?.ok_or_else(|| anyhow::anyhow!("新增商品失敗"))
}

pub fn update_product(conn: &Connection, id: i64, name: Option<&str>, price: Option<i64>, stock: Option<i64>, description: Option<&str>) -> Result<bool> {
    let existing = match get_product(conn, id)? {
        Some(p) => p,
        None => return Ok(false),
    };
    conn.execute(
        "UPDATE products SET name=?1, price=?2, stock=?3, description=?4, updated_at=datetime('now') WHERE id=?5",
        params![name.unwrap_or(&existing.name), price.unwrap_or(existing.price), stock.unwrap_or(existing.stock), description.or(existing.description.as_deref()), id],
    )?;
    Ok(true)
}

pub fn remove_product(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM products WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn get_product(conn: &Connection, id: i64) -> Result<Option<Product>> {
    let mut stmt = conn.prepare("SELECT id, shop_id, name, description, price, stock, image, created_at, updated_at FROM products WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Product {
            id: row.get(0)?,
            shop_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            image: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?;
    match rows.next() {
        Some(Ok(p)) => Ok(Some(p)),
        _ => Ok(None),
    }
}

pub fn list_products(conn: &Connection, shop_id: i64) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT id, shop_id, name, description, price, stock, image, created_at, updated_at FROM products WHERE shop_id = ?1 ORDER BY created_at DESC")?;
    let rows = stmt.query_map(params![shop_id], |row| {
        Ok(Product {
            id: row.get(0)?,
            shop_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            image: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?;
    let mut products = Vec::new();
    for row in rows {
        products.push(row?);
    }
    Ok(products)
}

pub fn search_products(conn: &Connection, q: Option<&str>, min_price: Option<i64>, max_price: Option<i64>) -> Result<Vec<ProductWithShop>> {
    let mut sql = String::from(
        "SELECT p.id, p.shop_id, s.name, s.user_id, p.name, p.description, p.price, p.stock, p.image, p.created_at
         FROM products p JOIN shops s ON s.id = p.shop_id WHERE p.stock > 0"
    );
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(q_str) = q {
        let idx = args.len() + 1;
        sql.push_str(&format!(" AND (p.name LIKE ?{} OR p.description LIKE ?{})", idx, idx));
        args.push(Box::new(format!("%{}%", q_str)));
    }
    if let Some(min) = min_price {
        let idx = args.len() + 1;
        sql.push_str(&format!(" AND p.price >= ?{}", idx));
        args.push(Box::new(min));
    }
    if let Some(max) = max_price {
        let idx = args.len() + 1;
        sql.push_str(&format!(" AND p.price <= ?{}", idx));
        args.push(Box::new(max));
    }
    sql.push_str(" ORDER BY p.created_at DESC");
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(ProductWithShop {
            id: row.get(0)?,
            shop_id: row.get(1)?,
            shop_name: row.get(2)?,
            shop_user_id: row.get(3)?,
            name: row.get(4)?,
            description: row.get(5)?,
            price: row.get(6)?,
            stock: row.get(7)?,
            image: row.get(8)?,
            created_at: row.get(9)?,
        })
    })?;
    let mut products = Vec::new();
    for row in rows {
        products.push(row?);
    }
    Ok(products)
}

pub fn decrement_stock(conn: &Connection, product_id: i64, quantity: i64) -> Result<bool> {
    let n = conn.execute(
        "UPDATE products SET stock = stock - ?1, updated_at = datetime('now') WHERE id = ?2 AND stock >= ?1",
        params![quantity, product_id],
    )?;
    Ok(n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::{shop, user};

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    fn setup_shop(c: &Connection) -> (i64, i64) {
        let uid = user::create_user(c, "alice", "Alice", None, None).unwrap();
        let s = shop::open_shop(c, uid, "Alice 的小店", None).unwrap();
        (uid, s.id)
    }

    #[test]
    fn test_add_and_list() {
        let c = conn();
        let (_uid, sid) = setup_shop(&c);
        let p = add_product(&c, sid, "咖啡豆", 299, 10, Some("衣索比亞")).unwrap();
        assert_eq!(p.price, 299);
        let list = list_products(&c, sid).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_search() {
        let c = conn();
        let (uid1, sid1) = setup_shop(&c);
        let uid2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let s2 = shop::open_shop(&c, uid2, "Bob 的店", None).unwrap();
        add_product(&c, sid1, "咖啡豆", 299, 10, None).unwrap();
        add_product(&c, s2.id, "茶葉", 199, 20, None).unwrap();

        let res = search_products(&c, Some("咖啡"), None, None).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].shop_name, "Alice 的小店");

        let res = search_products(&c, None, Some(200), Some(300)).unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_decrement_stock() {
        let c = conn();
        let (_uid, sid) = setup_shop(&c);
        let p = add_product(&c, sid, "咖啡豆", 299, 5, None).unwrap();
        assert!(decrement_stock(&c, p.id, 2).unwrap());
        let p2 = get_product(&c, p.id).unwrap().unwrap();
        assert_eq!(p2.stock, 3);
        assert!(!decrement_stock(&c, p.id, 10).unwrap());
    }
}
