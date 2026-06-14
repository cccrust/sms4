pub mod auth;
pub mod block;
pub mod follow;
pub mod interest;
pub mod like;
pub mod message;
pub mod order;
pub mod post;
pub mod product;
pub mod shop_message;
pub mod profile;
pub mod shop;
pub mod user;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_routes() -> Router<crate::web::AppState> {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/block", post(block::block).delete(block::unblock))
        .route("/block/{user_id}", get(block::list))
        .route("/block/{user_id}/{other_id}", get(block::check))
        .route("/users", get(user::list).post(user::create))
        .route(
            "/users/{id}",
            get(user::get).put(user::update).delete(user::delete),
        )
        .route("/users/{id}/timeline", get(user::timeline))
        .route("/users/{id}/followers", get(follow::followers))
        .route("/users/{id}/following", get(follow::following))
        .route("/posts", get(post::list).post(post::create))
        .route("/posts/{id}", get(post::get).delete(post::delete))
        .route("/posts/{id}/reply", post(post::reply))
        .route("/follow", post(follow::follow).delete(follow::unfollow))
        .route("/likes", post(like::like).delete(like::unlike))
        .route("/messages/send", post(message::send))
        .route("/messages/{user_id}/conversations", get(message::conversations))
        .route("/messages/{user_id}/unread", get(message::unread))
        .route("/messages/{user_id}/{other_id}", get(message::messages))
        .route("/profiles/{user_id}", get(profile::get).put(profile::update))
        .route("/profiles/search", get(profile::search))
        .route("/interests", post(interest::add).delete(interest::remove))
        .route("/interests/{user_id}", get(interest::list))
        .route("/shops/open", post(shop::open))
        .route("/shops", get(shop::my_shop))
        .route("/shops/{id}", get(shop::get).put(shop::update))
        .route("/shops/close", post(shop::close))
        .route("/products/search", get(product::search))
        .route("/products/shop/{shop_id}", get(product::list_by_shop).post(product::add))
        .route("/products/{id}", get(product::get).delete(product::remove))
        .route("/products/{id}/update", put(product::update))
        .route("/orders", post(order::create).get(order::list))
        .route("/orders/{id}", get(order::get))
        .route("/shop-messages/send", post(shop_message::send))
        .route("/shop-messages/{shop_id}", get(shop_message::list))
        .route("/shop-messages/conversations", get(shop_message::conversations))
}
