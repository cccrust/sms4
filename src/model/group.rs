use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub member_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupWithOwner {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub owner_username: String,
    pub owner_display_name: String,
    pub member_count: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupMember {
    pub id: i64,
    pub group_id: i64,
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupMemberBrief {
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
}

pub fn create(conn: &Connection, owner_id: i64, name: &str, description: Option<&str>) -> Result<Group> {
    conn.execute(
        "INSERT INTO groups (name, description, owner_id) VALUES (?1, ?2, ?3)",
        params![name, description, owner_id],
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO group_members (group_id, user_id, role) VALUES (?1, ?2, 'owner')",
        params![id, owner_id],
    )?;
    get(conn, id)?.ok_or_else(|| anyhow::anyhow!("建立社團失敗"))
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<Group>> {
    let mut stmt = conn.prepare("SELECT id, name, description, owner_id, member_count, created_at, updated_at FROM groups WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            owner_id: row.get(3)?,
            member_count: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(g)) => Ok(Some(g)),
        _ => Ok(None),
    }
}

pub fn list(conn: &Connection, search: Option<&str>) -> Result<Vec<GroupWithOwner>> {
    let mut sql = String::from(
        "SELECT g.id, g.name, g.description, g.owner_id, u.username, u.display_name, g.member_count, g.created_at
         FROM groups g JOIN users u ON u.id = g.owner_id"
    );
    if let Some(q) = search {
        sql.push_str(" WHERE g.name LIKE ?1 OR g.description LIKE ?1");
    }
    sql.push_str(" ORDER BY g.created_at DESC");
    let mut stmt = conn.prepare(&sql)?;
    let rows = if let Some(q) = search {
        let pattern = format!("%{}%", q);
        stmt.query_map(params![pattern], map_group_with_owner)?
    } else {
        stmt.query_map([], map_group_with_owner)?
    };
    let mut groups = Vec::new();
    for row in rows {
        groups.push(row?);
    }
    Ok(groups)
}

fn map_group_with_owner(row: &rusqlite::Row) -> rusqlite::Result<GroupWithOwner> {
    Ok(GroupWithOwner {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        owner_id: row.get(3)?,
        owner_username: row.get(4)?,
        owner_display_name: row.get(5)?,
        member_count: row.get(6)?,
        created_at: row.get(7)?,
    })
}

pub fn list_my(conn: &Connection, user_id: i64) -> Result<Vec<GroupWithOwner>> {
    let mut stmt = conn.prepare(
        "SELECT g.id, g.name, g.description, g.owner_id, u.username, u.display_name, g.member_count, g.created_at
         FROM groups g JOIN users u ON u.id = g.owner_id JOIN group_members gm ON gm.group_id = g.id
         WHERE gm.user_id = ?1 ORDER BY gm.joined_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], map_group_with_owner)?;
    let mut groups = Vec::new();
    for row in rows {
        groups.push(row?);
    }
    Ok(groups)
}

pub fn update(conn: &Connection, id: i64, user_id: i64, name: Option<&str>, description: Option<&str>) -> Result<bool> {
    let role = get_member_role(conn, id, user_id)?;
    if role != Some("owner".to_string()) && role != Some("admin".to_string()) {
        bail!("只有社團管理員可以編輯");
    }
    let existing = match get(conn, id)? {
        Some(g) => g,
        None => return Ok(false),
    };
    conn.execute(
        "UPDATE groups SET name=?1, description=?2, updated_at=datetime('now') WHERE id=?3",
        params![name.unwrap_or(&existing.name), description.or(existing.description.as_deref()), id],
    )?;
    Ok(true)
}

pub fn delete(conn: &Connection, id: i64, user_id: i64) -> Result<bool> {
    let g = match get(conn, id)? {
        Some(g) => g,
        None => return Ok(false),
    };
    if g.owner_id != user_id {
        bail!("只有社團創建者可以刪除");
    }
    conn.execute("DELETE FROM groups WHERE id = ?1", params![id])?;
    Ok(true)
}

pub fn join(conn: &Connection, group_id: i64, user_id: i64) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM group_members WHERE group_id = ?1 AND user_id = ?2",
        params![group_id, user_id],
        |row| row.get(0),
    )?;
    if exists {
        bail!("已經加入此社團");
    }
    conn.execute(
        "INSERT INTO group_members (group_id, user_id, role) VALUES (?1, ?2, 'member')",
        params![group_id, user_id],
    )?;
    conn.execute(
        "UPDATE groups SET member_count = member_count + 1 WHERE id = ?1",
        params![group_id],
    )?;
    Ok(true)
}

pub fn leave(conn: &Connection, group_id: i64, user_id: i64) -> Result<bool> {
    let g = match get(conn, group_id)? {
        Some(g) => g,
        None => return Ok(false),
    };
    if g.owner_id == user_id {
        bail!("創建者不能退出社團，請刪除社團");
    }
    let n = conn.execute(
        "DELETE FROM group_members WHERE group_id = ?1 AND user_id = ?2",
        params![group_id, user_id],
    )?;
    if n > 0 {
        conn.execute(
            "UPDATE groups SET member_count = member_count - 1 WHERE id = ?1",
            params![group_id],
        )?;
    }
    Ok(n > 0)
}

pub fn list_members(conn: &Connection, group_id: i64) -> Result<Vec<GroupMemberBrief>> {
    let mut stmt = conn.prepare(
        "SELECT gm.user_id, u.username, u.display_name, gm.role
         FROM group_members gm JOIN users u ON u.id = gm.user_id
         WHERE gm.group_id = ?1 ORDER BY gm.joined_at ASC"
    )?;
    let rows = stmt.query_map(params![group_id], |row| {
        Ok(GroupMemberBrief {
            user_id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            role: row.get(3)?,
        })
    })?;
    let mut members = Vec::new();
    for row in rows {
        members.push(row?);
    }
    Ok(members)
}

pub fn is_member(conn: &Connection, group_id: i64, user_id: i64) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM group_members WHERE group_id = ?1 AND user_id = ?2",
        params![group_id, user_id],
        |row| row.get(0),
    )?;
    Ok(exists)
}

fn get_member_role(conn: &Connection, group_id: i64, user_id: i64) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT role FROM group_members WHERE group_id = ?1 AND user_id = ?2")?;
    let mut rows = stmt.query_map(params![group_id, user_id], |row| row.get::<_, String>(0))?;
    match rows.next() {
        Some(Ok(r)) => Ok(Some(r)),
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
    fn test_create_and_get() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let g = create(&c, uid, "攝影愛好者", Some("喜歡拍照的朋友一起來")).unwrap();
        assert_eq!(g.name, "攝影愛好者");
        assert_eq!(g.member_count, 1);
        let got = get(&c, g.id).unwrap().unwrap();
        assert_eq!(got.id, g.id);
    }

    #[test]
    fn test_join_and_leave() {
        let c = conn();
        let uid1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let uid2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let g = create(&c, uid1, "測試社團", None).unwrap();
        assert!(join(&c, g.id, uid2).unwrap());
        assert_eq!(get(&c, g.id).unwrap().unwrap().member_count, 2);
        assert!(leave(&c, g.id, uid2).unwrap());
        assert_eq!(get(&c, g.id).unwrap().unwrap().member_count, 1);
    }

    #[test]
    fn test_duplicate_join_fails() {
        let c = conn();
        let uid1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let uid2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let g = create(&c, uid1, "社團", None).unwrap();
        join(&c, g.id, uid2).unwrap();
        assert!(join(&c, g.id, uid2).is_err());
    }

    #[test]
    fn test_owner_cannot_leave() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let g = create(&c, uid, "社團", None).unwrap();
        assert!(leave(&c, g.id, uid).is_err());
    }
}
