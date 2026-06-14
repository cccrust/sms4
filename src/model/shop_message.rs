use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ShopMessage {
    pub id: i64,
    pub shop_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShopMessageWithUser {
    pub id: i64,
    pub shop_id: i64,
    pub shop_name: String,
    pub sender_id: i64,
    pub sender_username: String,
    pub sender_display_name: String,
    pub receiver_id: i64,
    pub receiver_username: String,
    pub receiver_display_name: String,
    pub content: String,
    pub created_at: String,
}

pub fn send(conn: &Connection, shop_id: i64, sender_id: i64, receiver_id: i64, content: &str) -> Result<ShopMessage> {
    conn.execute(
        "INSERT INTO shop_messages (shop_id, sender_id, receiver_id, content) VALUES (?1, ?2, ?3, ?4)",
        params![shop_id, sender_id, receiver_id, content],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)?.ok_or_else(|| anyhow::anyhow!("傳送失敗"))
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<ShopMessage>> {
    let mut stmt = conn.prepare("SELECT id, shop_id, sender_id, receiver_id, content, created_at FROM shop_messages WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(ShopMessage {
            id: row.get(0)?,
            shop_id: row.get(1)?,
            sender_id: row.get(2)?,
            receiver_id: row.get(3)?,
            content: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(m)) => Ok(Some(m)),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ShopConversation {
    pub shop_id: i64,
    pub shop_name: String,
    pub other_id: i64,
    pub last_message: Option<String>,
    pub last_message_at: Option<String>,
}

pub fn list_for_shop(conn: &Connection, shop_id: i64, user_id: i64, other_id: i64) -> Result<Vec<ShopMessageWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT sm.id, sm.shop_id, s.name,
                sm.sender_id, su.username, su.display_name,
                sm.receiver_id, ru.username, ru.display_name,
                sm.content, sm.created_at
         FROM shop_messages sm
         JOIN shops s ON s.id = sm.shop_id
         JOIN users su ON su.id = sm.sender_id
         JOIN users ru ON ru.id = sm.receiver_id
         WHERE sm.shop_id = ?1
           AND ((sm.sender_id = ?2 AND sm.receiver_id = ?3)
             OR (sm.sender_id = ?3 AND sm.receiver_id = ?2))
         ORDER BY sm.created_at ASC"
    )?;
    let rows = stmt.query_map(params![shop_id, user_id, other_id], |row| {
        Ok(ShopMessageWithUser {
            id: row.get(0)?,
            shop_id: row.get(1)?,
            shop_name: row.get(2)?,
            sender_id: row.get(3)?,
            sender_username: row.get(4)?,
            sender_display_name: row.get(5)?,
            receiver_id: row.get(6)?,
            receiver_username: row.get(7)?,
            receiver_display_name: row.get(8)?,
            content: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    let mut msgs = Vec::new();
    for row in rows {
        msgs.push(row?);
    }
    Ok(msgs)
}

pub fn list_conversations(conn: &Connection, user_id: i64) -> Result<Vec<ShopConversation>> {
    let mut stmt = conn.prepare(
        "SELECT sm.shop_id, s.name,
                CASE WHEN sm.sender_id = ?1 THEN sm.receiver_id ELSE sm.sender_id END AS other_id,
                (SELECT content FROM shop_messages WHERE shop_id = sm.shop_id
                 AND ((sender_id = ?1 AND receiver_id = other_id) OR (sender_id = other_id AND receiver_id = ?1))
                 ORDER BY created_at DESC LIMIT 1) AS last_msg,
                (SELECT created_at FROM shop_messages WHERE shop_id = sm.shop_id
                 AND ((sender_id = ?1 AND receiver_id = other_id) OR (sender_id = other_id AND receiver_id = ?1))
                 ORDER BY created_at DESC LIMIT 1) AS last_at
         FROM shop_messages sm
         WHERE sm.sender_id = ?1 OR sm.receiver_id = ?1
         GROUP BY sm.shop_id, other_id
         ORDER BY last_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(ShopConversation {
            shop_id: row.get(0)?,
            shop_name: row.get(1)?,
            other_id: row.get(2)?,
            last_message: row.get(3)?,
            last_message_at: row.get(4)?,
        })
    })?;
    let mut convs = Vec::new();
    for row in rows {
        convs.push(row?);
    }
    Ok(convs)
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
    fn test_send_and_list() {
        let c = conn();
        let uid1 = user::create_user(&c, "alice", "Alice", None, None).unwrap();
        let uid2 = user::create_user(&c, "bob", "Bob", None, None).unwrap();
        let s = shop::open_shop(&c, uid1, "Alice 的小店", None).unwrap();

        let msg = send(&c, s.id, uid2, uid1, "請問有營業嗎？").unwrap();
        assert_eq!(msg.content, "請問有營業嗎？");

        let reply = send(&c, s.id, uid1, uid2, "有的，每天 10-20").unwrap();
        assert_eq!(reply.content, "有的，每天 10-20");

        let msgs = list_for_shop(&c, s.id, uid2, uid1).unwrap();
        assert_eq!(msgs.len(), 2);
    }
}
