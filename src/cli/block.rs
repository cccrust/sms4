use crate::cli::fmt;
use crate::model::block;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct BlockCommand {
    #[command(subcommand)]
    pub subcommand: BlockSubcommands,
}

#[derive(Subcommand)]
pub enum BlockSubcommands {
    Add {
        blocker_id: i64,
        blocked_id: i64,
    },
    Remove {
        blocker_id: i64,
        blocked_id: i64,
    },
    List {
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &BlockSubcommands) -> Result<()> {
    match cmd {
        BlockSubcommands::Add { blocker_id, blocked_id } => {
            match block::block_user(conn, *blocker_id, *blocked_id) {
                Ok(()) => println!(
                    "{}",
                    fmt::success_msg(&format!("#{} 已封鎖 #{}", blocker_id, blocked_id))
                ),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        BlockSubcommands::Remove { blocker_id, blocked_id } => {
            if block::unblock_user(conn, *blocker_id, *blocked_id)? {
                println!(
                    "{}",
                    fmt::success_msg(&format!("#{} 已解除封鎖 #{}", blocker_id, blocked_id))
                );
            } else {
                println!("{}", fmt::error_msg("封鎖記錄不存在"));
            }
        }
        BlockSubcommands::List { user_id } => {
            let users = block::list_blocked_users(conn, *user_id)?;
            if users.is_empty() {
                println!("{}", fmt::info_msg("尚未封鎖任何人。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:<25}", "ID", "帳號", "顯示名稱")));
            println!("{}", "-".repeat(55));
            for u in &users {
                println!("{:<4} @{:<18} {:<25}", u.id, u.username, u.display_name);
            }
        }
    }
    Ok(())
}
