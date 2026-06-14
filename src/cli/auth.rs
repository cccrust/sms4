use crate::cli::fmt;
use crate::model::auth;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub subcommand: AuthSubcommands,
}

#[derive(Subcommand)]
pub enum AuthSubcommands {
    Register {
        username: String,
        password: String,
        #[arg(long, short)]
        name: Option<String>,
    },
    Login {
        username: String,
        password: String,
    },
    Logout {
        token: String,
    },
}

pub fn run(conn: &Connection, cmd: &AuthSubcommands) -> Result<()> {
    match cmd {
        AuthSubcommands::Register { username, password, name } => {
            let display_name = name.clone().unwrap_or_else(|| username.clone());
            match auth::register(conn, username, password, &display_name) {
                Ok(id) => {
                    println!("{}", fmt::success_msg(&format!("已註冊使用者 #{}: @{} ({})", id, username, display_name)));
                }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("UNIQUE") {
                        println!("{}", fmt::error_msg(&format!("帳號 @{} 已被使用", username)));
                    } else {
                        println!("{}", fmt::error_msg(&msg));
                    }
                }
            }
        }
        AuthSubcommands::Login { username, password } => {
            match auth::login(conn, username, password) {
                Ok(token) => {
                    println!("{}", fmt::success_msg(&format!("登入成功！Token: {}", token)));
                }
                Err(e) => {
                    println!("{}", fmt::error_msg(&e.to_string()));
                }
            }
        }
        AuthSubcommands::Logout { token } => {
            if auth::logout(conn, token)? {
                println!("{}", fmt::success_msg("已登出。"));
            } else {
                println!("{}", fmt::error_msg("Token 無效"));
            }
        }
    }
    Ok(())
}
