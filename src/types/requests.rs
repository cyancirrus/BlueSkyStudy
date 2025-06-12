use crate::types::aliases::UserId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    pub follower: UserId,
    pub followee: UserId,
}

#[derive(Debug, Deserialize)]
pub struct PublishRequest {
    pub user: UserId,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct NewsfeedRequest {
    pub user: UserId,
}

#[derive(Debug, Deserialize)]
pub struct UnfollowRequest {
    pub follower: UserId,
    pub followee: UserId,
    pub reason: String,
}
