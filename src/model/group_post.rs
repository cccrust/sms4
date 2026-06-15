use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GroupPost {
    pub id: i64,
    pub group_id: i64,
    pub user_id: i64,
    pub content: String,
    pub likes_count: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupPostWithUser {
    pub id: i64,
    pub group_id: i64,
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub content: String,
    pub likes_count: i64,
    pub created_at: String,
}

pub fn add(conn: &Connection, group_id: i64, user_id: i64, content: &str) -> Result<GroupPost> {
    if !crate::model::group::is_member(conn, group_id, user_id)? {
        anyhow::bail!("只有社團成員可以發文");
    }
    conn.execute(
        "INSERT INTO group_posts (group_id, user_id, content) VALUES (?1, ?2, ?3)",
        params![group_id, user_id, content],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)?.ok_or_else(|| anyhow::anyhow!("發文失敗"))
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<GroupPost>> {
    let mut stmt = conn.prepare("SELECT id, group_id, user_id, content, likes_count, created_at FROM group_posts WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(GroupPost {
            id: row.get(0)?,
            group_id: row.get(1)?,
            user_id: row.get(2)?,
            content: row.get(3)?,
            likes_count: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(p)) => Ok(Some(p)),
        _ => Ok(None),
    }
}

pub fn list(conn: &Connection, group_id: i64) -> Result<Vec<GroupPostWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT gp.id, gp.group_id, gp.user_id, u.username, u.display_name, gp.content, gp.likes_count, gp.created_at
         FROM group_posts gp JOIN users u ON u.id = gp.user_id
         WHERE gp.group_id = ?1 ORDER BY gp.created_at DESC"
    )?;
    let rows = stmt.query_map(params![group_id], |row| {
        Ok(GroupPostWithUser {
            id: row.get(0)?,
            group_id: row.get(1)?,
            user_id: row.get(2)?,
            username: row.get(3)?,
            display_name: row.get(4)?,
            content: row.get(5)?,
            likes_count: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;
    let mut posts = Vec::new();
    for row in rows {
        posts.push(row?);
    }
    Ok(posts)
}

pub fn delete(conn: &Connection, post_id: i64, user_id: i64) -> Result<bool> {
    let post = match get(conn, post_id)? {
        Some(p) => p,
        None => return Ok(false),
    };
    let group = crate::model::group::get(conn, post.group_id)?.unwrap();
    if post.user_id != user_id && group.owner_id != user_id {
        anyhow::bail!("只有作者或社團創建者可以刪除");
    }
    let n = conn.execute("DELETE FROM group_posts WHERE id = ?1", params![post_id])?;
    Ok(n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::{group, user};

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_add_and_list() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let g = group::create(&c, uid, "社團", None).unwrap();
        let p = add(&c, g.id, uid, "大家好！").unwrap();
        assert_eq!(p.content, "大家好！");
        let posts = list(&c, g.id).unwrap();
        assert_eq!(posts.len(), 1);
    }

    #[test]
    fn test_non_member_cannot_post() {
        let c = conn();
        let uid1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let uid2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let g = group::create(&c, uid1, "社團", None).unwrap();
        assert!(add(&c, g.id, uid2, "hello").is_err());
    }

    #[test]
    fn test_delete() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let g = group::create(&c, uid, "社團", None).unwrap();
        let p = add(&c, g.id, uid, "test").unwrap();
        assert!(delete(&c, p.id, uid).unwrap());
        assert!(get(&c, p.id).unwrap().is_none());
    }
}
