use crate::cli::fmt;
use crate::model::product;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct ProductCommand {
    #[command(subcommand)]
    pub subcommand: ProductSubcommands,
}

#[derive(Subcommand)]
pub enum ProductSubcommands {
    Add {
        shop_id: i64,
        name: String,
        price: i64,
        #[arg(long)]
        stock: Option<i64>,
        #[arg(long)]
        description: Option<String>,
    },
    Update {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        price: Option<i64>,
        #[arg(long)]
        stock: Option<i64>,
        #[arg(long)]
        description: Option<String>,
    },
    Remove {
        id: i64,
    },
    List {
        shop_id: i64,
    },
    Search {
        #[arg(long, short)]
        q: Option<String>,
        #[arg(long)]
        min_price: Option<i64>,
        #[arg(long)]
        max_price: Option<i64>,
    },
}

pub fn run(conn: &Connection, cmd: &ProductSubcommands) -> Result<()> {
    match cmd {
        ProductSubcommands::Add { shop_id, name, price, stock, description } => {
            match product::add_product(conn, *shop_id, name, *price, stock.unwrap_or(0), description.as_deref()) {
                Ok(p) => println!("{}", fmt::success_msg(&format!("商品 #{}「{}」已上架（${}）", p.id, p.name, p.price))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ProductSubcommands::Update { id, name, price, stock, description } => {
            match product::update_product(conn, *id, name.as_deref(), *price, *stock, description.as_deref()) {
                Ok(true) => println!("{}", fmt::success_msg(&format!("商品 #{} 已更新。", id))),
                Ok(false) => println!("{}", fmt::error_msg("商品不存在")),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ProductSubcommands::Remove { id } => {
            if product::remove_product(conn, *id)? {
                println!("{}", fmt::success_msg(&format!("商品 #{} 已刪除。", id)));
            } else {
                println!("{}", fmt::error_msg("商品不存在"));
            }
        }
        ProductSubcommands::List { shop_id } => {
            let products = product::list_products(conn, *shop_id)?;
            if products.is_empty() {
                println!("{}", fmt::info_msg("此商店尚無商品。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<25} {:>8} {:>5} {}", "ID", "名稱", "價格", "庫存", "描述")));
            println!("{}", "-".repeat(80));
            for p in &products {
                println!("{:<4} {:<25} ${:>6} {:>5} {}",
                    p.id, p.name, p.price, p.stock, p.description.as_deref().unwrap_or(""));
            }
        }
        ProductSubcommands::Search { q, min_price, max_price } => {
            let products = product::search_products(conn, q.as_deref(), *min_price, *max_price)?;
            if products.is_empty() {
                println!("{}", fmt::info_msg("查無符合條件的商品。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:<15} {:>8} {}", "ID", "商品", "商店", "價格", "庫存")));
            println!("{}", "-".repeat(75));
            for p in &products {
                println!("{:<4} {:<20} {:<15} ${:>6} {:>5}", p.id, p.name, p.shop_name, p.price, p.stock);
            }
        }
    }
    Ok(())
}
