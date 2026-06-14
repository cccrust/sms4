use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Shop {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn open_shop(conn: &Connection, user_id: i64, name: &str, description: Option<&str>) -> Result<Shop> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM shops WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )?;
    if exists {
        bail!("該使用者已經有商店了");
    }
    conn.execute(
        "INSERT INTO shops (user_id, name, description) VALUES (?1, ?2, ?3)",
        params![user_id, name, description],
    )?;
    get_shop_by_user(conn, user_id)?.ok_or_else(|| anyhow::anyhow!("開店失敗"))
}

pub fn update_shop(conn: &Connection, user_id: i64, name: Option<&str>, description: Option<&str>) -> Result<bool> {
    let existing = match get_shop_by_user(conn, user_id)? {
        Some(s) => s,
        None => return Ok(false),
    };
    conn.execute(
        "UPDATE shops SET name=?1, description=?2, updated_at=datetime('now') WHERE user_id=?3",
        params![name.unwrap_or(&existing.name), description.or(existing.description.as_deref()), user_id],
    )?;
    Ok(true)
}

pub fn close_shop(conn: &Connection, user_id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM shops WHERE user_id = ?1", params![user_id])?;
    Ok(n > 0)
}

pub fn get_shop_by_user(conn: &Connection, user_id: i64) -> Result<Option<Shop>> {
    let mut stmt = conn.prepare("SELECT id, user_id, name, description, created_at, updated_at FROM shops WHERE user_id = ?1")?;
    let mut rows = stmt.query_map(params![user_id], |row| {
        Ok(Shop {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn get_shop_by_id(conn: &Connection, id: i64) -> Result<Option<Shop>> {
    let mut stmt = conn.prepare("SELECT id, user_id, name, description, created_at, updated_at FROM shops WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Shop {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::user;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_open_and_get() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let shop = open_shop(&c, uid, "Alice 的小店", Some("歡迎光臨")).unwrap();
        assert_eq!(shop.name, "Alice 的小店");

        let got = get_shop_by_user(&c, uid).unwrap().unwrap();
        assert_eq!(got.id, shop.id);
    }

    #[test]
    fn test_duplicate_shop_fails() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        open_shop(&c, uid, "Shop", None).unwrap();
        assert!(open_shop(&c, uid, "Shop 2", None).is_err());
    }

    #[test]
    fn test_close() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        open_shop(&c, uid, "Shop", None).unwrap();
        assert!(close_shop(&c, uid).unwrap());
        assert!(get_shop_by_user(&c, uid).unwrap().is_none());
    }
}
