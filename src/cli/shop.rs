use crate::cli::fmt;
use crate::model::shop;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct ShopCommand {
    #[command(subcommand)]
    pub subcommand: ShopSubcommands,
}

#[derive(Subcommand)]
pub enum ShopSubcommands {
    Open {
        user_id: i64,
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    Update {
        user_id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Show {
        user_id: i64,
    },
    Close {
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &ShopSubcommands) -> Result<()> {
    match cmd {
        ShopSubcommands::Open { user_id, name, description } => {
            match shop::open_shop(conn, *user_id, name, description.as_deref()) {
                Ok(s) => println!("{}", fmt::success_msg(&format!("商店 #{}「{}」已開張！", s.id, s.name))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ShopSubcommands::Update { user_id, name, description } => {
            match shop::update_shop(conn, *user_id, name.as_deref(), description.as_deref()) {
                Ok(true) => println!("{}", fmt::success_msg("商店資訊已更新。")),
                Ok(false) => println!("{}", fmt::error_msg("使用者沒有商店")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ShopSubcommands::Show { user_id } => {
            match shop::get_shop_by_user(conn, *user_id)? {
                Some(s) => {
                    println!("ID:       {}", s.id);
                    println!("名稱:     {}", s.name);
                    println!("描述:     {}", s.description.as_deref().unwrap_or("N/A"));
                    println!("建立時間: {}", s.created_at);
                }
                None => println!("{}", fmt::info_msg("該使用者沒有商店。")),
            }
        }
        ShopSubcommands::Close { user_id } => {
            if shop::close_shop(conn, *user_id)? {
                println!("{}", fmt::success_msg("商店已關閉。"));
            } else {
                println!("{}", fmt::error_msg("使用者沒有商店"));
            }
        }
    }
    Ok(())
}
