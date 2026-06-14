use crate::cli::fmt;
use crate::model::order;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct OrderCommand {
    #[command(subcommand)]
    pub subcommand: OrderSubcommands,
}

#[derive(Subcommand)]
pub enum OrderSubcommands {
    Buy {
        product_id: i64,
        buyer_id: i64,
        #[arg(long, default_value_t = 1)]
        quantity: i64,
    },
    List {
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &OrderSubcommands) -> Result<()> {
    match cmd {
        OrderSubcommands::Buy { product_id, buyer_id, quantity } => {
            match order::create_order(conn, *buyer_id, *product_id, *quantity) {
                Ok(o) => println!("{}", fmt::success_msg(&format!("訂單 #{} 已建立（總額 ${}）", o.id, o.total_price))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        OrderSubcommands::List { user_id } => {
            let orders = order::list_orders(conn, *user_id)?;
            if orders.is_empty() {
                println!("{}", fmt::info_msg("尚無訂單。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:<10} {:>8} {:<6} {}", "ID", "商品", "商店", "總額", "狀態", "時間")));
            println!("{}", "-".repeat(80));
            for o in &orders {
                println!("{:<4} {:<20} {:<10} ${:>6} {:<6} {}",
                    o.id, o.product_name, o.shop_name, o.total_price, o.status, o.created_at);
            }
        }
    }
    Ok(())
}
