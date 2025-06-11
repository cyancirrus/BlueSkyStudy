use axum::{
    Router,
    routing::{get, post},
};
use crate::handlers::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(welcome_handler))
        .route("/follow", post(follow_handler))
        .route("/unfollow", post(unfollow_handler))
        .route("/publish", post(publish_handler))
        .route("/newsfeed", post(newsfeed_handler))
}
