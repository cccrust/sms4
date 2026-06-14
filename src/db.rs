use anyhow::Result;
use rusqlite::{params, Connection};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(include_str!("db.sql"))?;
    migrate(conn)?;
    Ok(())
}

fn migrate(conn: &Connection) -> Result<()> {
    // 為舊資料庫補上 password_hash 欄位
    let _ = conn.execute("ALTER TABLE users ADD COLUMN password_hash TEXT", []);
    Ok(())
}

pub fn set_password(conn: &Connection, user_id: i64, password_hash: &str) -> Result<()> {
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![password_hash, user_id],
    )?;
    Ok(())
}

pub fn get_password_hash(conn: &Connection, user_id: i64) -> Result<Option<String>> {
    let result = conn.query_row(
        "SELECT password_hash FROM users WHERE id = ?1",
        params![user_id],
        |row| row.get::<_, Option<String>>(0),
    );
    match result {
        Ok(hash) => Ok(hash),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
