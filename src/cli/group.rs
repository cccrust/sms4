use crate::cli::fmt;
use crate::model::group::{self, GroupWithOwner};
use crate::model::group_post;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct GroupCommand {
    #[command(subcommand)]
    pub subcommand: GroupSubcommands,
}

#[derive(Subcommand)]
pub enum GroupSubcommands {
    Create {
        user_id: i64,
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    List {
        #[arg(long, short)]
        search: Option<String>,
    },
    Mine {
        user_id: i64,
    },
    Get {
        id: i64,
    },
    Update {
        id: i64,
        user_id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Delete {
        id: i64,
        user_id: i64,
    },
    Join {
        group_id: i64,
        user_id: i64,
    },
    Leave {
        group_id: i64,
        user_id: i64,
    },
    Members {
        group_id: i64,
    },
    #[command(subcommand)]
    Post(GroupPostCommands),
}

#[derive(Subcommand)]
pub enum GroupPostCommands {
    Add {
        group_id: i64,
        user_id: i64,
        content: String,
    },
    List {
        group_id: i64,
    },
    Delete {
        post_id: i64,
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &GroupSubcommands) -> Result<()> {
    match cmd {
        GroupSubcommands::Create { user_id, name, description } => {
            match group::create(conn, *user_id, name, description.as_deref()) {
                Ok(g) => println!("{}", fmt::success_msg(&format!("社團 #{}「{}」已建立！", g.id, g.name))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        GroupSubcommands::List { search } => {
            let groups = group::list(conn, search.as_deref())?;
            if groups.is_empty() {
                println!("{}", fmt::info_msg("尚無社團"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:>8} {}", "ID", "名稱", "成員", "建立者")));
            println!("{}", "-".repeat(60));
            for g in &groups {
                println!("{:<4} {:<20} {:>8} {}", g.id, g.name, g.member_count, g.owner_display_name);
            }
        }
        GroupSubcommands::Mine { user_id } => {
            let groups = group::list_my(conn, *user_id)?;
            if groups.is_empty() {
                println!("{}", fmt::info_msg("你尚未加入任何社團"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:>8} {}", "ID", "名稱", "成員", "角色")));
            println!("{}", "-".repeat(60));
            for g in &groups {
                let role = if g.owner_id == *user_id { "owner" } else { "member" };
                println!("{:<4} {:<20} {:>8} {}", g.id, g.name, g.member_count, role);
            }
        }
        GroupSubcommands::Get { id } => {
            match group::get(conn, *id)? {
                Some(g) => {
                    println!("ID:       {}", g.id);
                    println!("名稱:     {}", g.name);
                    println!("描述:     {}", g.description.as_deref().unwrap_or("N/A"));
                    println!("創建者:   #{}", g.owner_id);
                    println!("成員數:   {}", g.member_count);
                    println!("建立時間: {}", g.created_at);
                }
                None => println!("{}", fmt::error_msg("社團不存在")),
            }
        }
        GroupSubcommands::Update { id, user_id, name, description } => {
            match group::update(conn, *id, *user_id, name.as_deref(), description.as_deref()) {
                Ok(true) => println!("{}", fmt::success_msg(&format!("社團 #{} 已更新。", id))),
                Ok(false) => println!("{}", fmt::error_msg("社團不存在")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        GroupSubcommands::Delete { id, user_id } => {
            match group::delete(conn, *id, *user_id) {
                Ok(true) => println!("{}", fmt::success_msg(&format!("社團 #{} 已刪除。", id))),
                Ok(false) => println!("{}", fmt::error_msg("社團不存在")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        GroupSubcommands::Join { group_id, user_id } => {
            match group::join(conn, *group_id, *user_id) {
                Ok(true) => println!("{}", fmt::success_msg("已加入社團！")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
                _ => unreachable!(),
            }
        }
        GroupSubcommands::Leave { group_id, user_id } => {
            match group::leave(conn, *group_id, *user_id) {
                Ok(true) => println!("{}", fmt::success_msg("已退出社團。")),
                Ok(false) => println!("{}", fmt::error_msg("你不在這個社團中")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        GroupSubcommands::Members { group_id } => {
            let members = group::list_members(conn, *group_id)?;
            if members.is_empty() {
                println!("{}", fmt::info_msg("社團沒有成員"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {}", "ID", "名稱", "角色")));
            println!("{}", "-".repeat(50));
            for m in &members {
                println!("{:<4} {:<20} {}", m.user_id, m.display_name, m.role);
            }
        }
        GroupSubcommands::Post(cmd2) => run_post(conn, cmd2)?,
    }
    Ok(())
}

fn run_post(conn: &Connection, cmd: &GroupPostCommands) -> Result<()> {
    match cmd {
        GroupPostCommands::Add { group_id, user_id, content } => {
            match group_post::add(conn, *group_id, *user_id, content) {
                Ok(p) => println!("{}", fmt::success_msg(&format!("貼文 #{} 已發布到社團！", p.id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        GroupPostCommands::List { group_id } => {
            let posts = group_post::list(conn, *group_id)?;
            if posts.is_empty() {
                println!("{}", fmt::info_msg("社團尚無貼文。"));
                return Ok(());
            }
            for p in &posts {
                println!("[#{}] {} (@{})", p.id, p.content, p.username);
                println!("      {} 讚", p.likes_count);
                println!("");
            }
        }
        GroupPostCommands::Delete { post_id, user_id } => {
            match group_post::delete(conn, *post_id, *user_id) {
                Ok(true) => println!("{}", fmt::success_msg(&format!("貼文 #{} 已刪除。", post_id))),
                Ok(false) => println!("{}", fmt::error_msg("貼文不存在")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
    }
    Ok(())
}
