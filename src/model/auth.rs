use crate::db;
use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_at: String,
}

pub fn register(
    conn: &Connection,
    username: &str,
    password: &str,
    display_name: &str,
) -> Result<i64> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    conn.execute(
        "INSERT INTO users (username, display_name, password_hash) VALUES (?1, ?2, ?3)",
        params![username, display_name, hash],
    )?;
    let id = conn.last_insert_rowid();
    Ok(id)
}

pub fn login(conn: &Connection, username: &str, password: &str) -> Result<String> {
    let mut stmt = conn.prepare(
        "SELECT id, password_hash FROM users WHERE username = ?1",
    )?;
    let row = stmt.query_row(params![username], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?))
    });

    let (user_id, password_hash) = match row {
        Ok(r) => r,
        Err(rusqlite::Error::QueryReturnedNoRows) => bail!("帳號或密碼錯誤"),
        Err(e) => return Err(e.into()),
    };

    let hash = match password_hash {
        Some(h) => h,
        None => bail!("帳號或密碼錯誤"),
    };

    if !bcrypt::verify(password, &hash)? {
        bail!("帳號或密碼錯誤");
    }

    let token = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO sessions (user_id, token) VALUES (?1, ?2)",
        params![user_id, token],
    )?;
    Ok(token)
}

pub fn logout(conn: &Connection, token: &str) -> Result<bool> {
    let n = conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
    Ok(n > 0)
}

pub fn get_user_by_token(conn: &Connection, token: &str) -> Result<Option<i64>> {
    let mut stmt = conn.prepare(
        "SELECT user_id FROM sessions WHERE token = ?1",
    )?;
    let mut rows = stmt.query_map(params![token], |row| row.get::<_, i64>(0))?;
    match rows.next() {
        Some(Ok(uid)) => Ok(Some(uid)),
        _ => Ok(None),
    }
}

pub fn is_username_taken(conn: &Connection, username: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM users WHERE username = ?1",
        params![username],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_register_and_login() {
        let c = conn();
        let id = register(&c, "alice", "secret123", "Alice").unwrap();
        assert!(id > 0);

        let token = login(&c, "alice", "secret123").unwrap();
        assert!(!token.is_empty());

        let uid = get_user_by_token(&c, &token).unwrap().unwrap();
        assert_eq!(uid, id);
    }

    #[test]
    fn test_login_wrong_password() {
        let c = conn();
        register(&c, "bob", "goodpass", "Bob").unwrap();
        assert!(login(&c, "bob", "wrongpass").is_err());
    }

    #[test]
    fn test_login_nonexistent_user() {
        let c = conn();
        assert!(login(&c, "nobody", "x").is_err());
    }

    #[test]
    fn test_logout() {
        let c = conn();
        register(&c, "carol", "pass", "Carol").unwrap();
        let token = login(&c, "carol", "pass").unwrap();
        assert!(get_user_by_token(&c, &token).unwrap().is_some());

        assert!(logout(&c, &token).unwrap());
        assert!(get_user_by_token(&c, &token).unwrap().is_none());
    }

    #[test]
    fn test_username_taken() {
        let c = conn();
        register(&c, "dave", "p", "Dave").unwrap();
        assert!(is_username_taken(&c, "dave").unwrap());
        assert!(!is_username_taken(&c, "eve").unwrap());
    }
}
