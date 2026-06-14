use clap::{Parser, Subcommand};

pub mod auth;
pub mod block;
pub mod fmt;
pub mod follow;
pub mod interest;
pub mod like;
pub mod message;
pub mod order;
pub mod post;
pub mod product;
pub mod profile;
pub mod shop;
pub mod user;

pub use auth::AuthCommand;
pub use block::BlockCommand;
pub use follow::FollowCommand;
pub use interest::InterestCommand;
pub use like::LikeCommand;
pub use message::MessageCommand;
pub use order::OrderCommand;
pub use post::PostCommand;
pub use product::ProductCommand;
pub use profile::ProfileCommand;
pub use shop::ShopCommand;
pub use user::UserCommand;

#[derive(Parser)]
#[command(name = "sms4", version, about = "SMS4 - 手機社群軟體 (Threads 風格)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Auth(AuthCommand),
    Block(BlockCommand),
    User(UserCommand),
    Post(PostCommand),
    Follow(FollowCommand),
    Like(LikeCommand),
    #[command(name = "msg")]
    Message(MessageCommand),
    Profile(ProfileCommand),
    Interest(InterestCommand),
    Shop(ShopCommand),
    Product(ProductCommand),
    Order(OrderCommand),
    Web {
        #[arg(long, default_value_t = 8080)]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long)]
        dev: bool,
    },
}
