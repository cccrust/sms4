use anyhow::{bail, Result};
use rusqlite::{params, Connection};

pub fn block_user(conn: &Connection, blocker_id: i64, blocked_id: i64) -> Result<()> {
    if blocker_id == blocked_id {
        bail!("不能封鎖自己");
    }
    conn.execute(
        "INSERT OR IGNORE INTO blocks (blocker_id, blocked_id) VALUES (?1, ?2)",
        params![blocker_id, blocked_id],
    )?;
    Ok(())
}

pub fn unblock_user(conn: &Connection, blocker_id: i64, blocked_id: i64) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM blocks WHERE blocker_id = ?1 AND blocked_id = ?2",
        params![blocker_id, blocked_id],
    )?;
    Ok(n > 0)
}

pub fn get_blocked_ids(conn: &Connection, user_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare("SELECT blocked_id FROM blocks WHERE blocker_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| row.get::<_, i64>(0))?;
    let mut ids = Vec::new();
    for row in rows {
        ids.push(row?);
    }
    Ok(ids)
}

pub fn get_blocker_ids(conn: &Connection, user_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare("SELECT blocker_id FROM blocks WHERE blocked_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| row.get::<_, i64>(0))?;
    let mut ids = Vec::new();
    for row in rows {
        ids.push(row?);
    }
    Ok(ids)
}

pub fn is_blocked(conn: &Connection, user_id: i64, other_id: i64) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM blocks WHERE blocker_id = ?1 AND blocked_id = ?2",
        params![user_id, other_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn either_blocked(conn: &Connection, a_id: i64, b_id: i64) -> Result<bool> {
    Ok(is_blocked(conn, a_id, b_id)? || is_blocked(conn, b_id, a_id)?)
}

pub fn list_blocked_users(conn: &Connection, user_id: i64) -> Result<Vec<crate::model::user::UserBrief>> {
    let mut stmt = conn.prepare(
        "SELECT u.id, u.username, u.display_name FROM blocks b
         JOIN users u ON u.id = b.blocked_id
         WHERE b.blocker_id = ?1
         ORDER BY b.created_at DESC",
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(crate::model::user::UserBrief {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
        })
    })?;
    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }
    Ok(users)
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
    fn test_block_unblock() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();

        block_user(&c, u1, u2).unwrap();
        assert!(is_blocked(&c, u1, u2).unwrap());

        assert!(unblock_user(&c, u1, u2).unwrap());
        assert!(!is_blocked(&c, u1, u2).unwrap());
    }

    #[test]
    fn test_block_self_fails() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        assert!(block_user(&c, u1, u1).is_err());
    }

    #[test]
    fn test_either_blocked() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();

        block_user(&c, u1, u2).unwrap();
        assert!(either_blocked(&c, u1, u2).unwrap());
        assert!(either_blocked(&c, u2, u1).unwrap());
    }

    #[test]
    fn test_list_blocked() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let u3 = user::create_user(&c, "carol", "Carol", None, None).unwrap();

        block_user(&c, u1, u2).unwrap();
        block_user(&c, u1, u3).unwrap();

        let blocked = list_blocked_users(&c, u1).unwrap();
        assert_eq!(blocked.len(), 2);
    }
}
