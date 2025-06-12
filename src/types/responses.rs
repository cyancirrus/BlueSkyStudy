use crate::state::Post;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WelcomeMessage {
    pub msg: &'static str,
}

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct UnfollowResponse {
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct NewsfeedResponse {
    pub status: &'static str,
    pub feed: Vec<Post>,
}
