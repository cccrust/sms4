use clap::{Parser, Subcommand};

pub mod fmt;
pub mod follow;
pub mod like;
pub mod post;
pub mod user;

pub use follow::FollowCommand;
pub use like::LikeCommand;
pub use post::PostCommand;
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
    User(UserCommand),
    Post(PostCommand),
    Follow(FollowCommand),
    Like(LikeCommand),
}
