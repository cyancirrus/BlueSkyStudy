use crate::handlers::*;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn create_router(logic: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(welcome_handler))
        .route("/follow", post(follow_handler))
        .route("/unfollow", post(unfollow_handler))
        .route("/publish", post(publish_handler))
        .route("/newsfeed", post(newsfeed_handler))
        .with_state(logic)
}
