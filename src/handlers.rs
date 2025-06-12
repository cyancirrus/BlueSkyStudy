use crate::types::aliases::SharedAppState;
use crate::types::requests::*;
use crate::types::responses::*;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

pub async fn welcome_handler() -> (StatusCode, Json<WelcomeMessage>) {
    (
        StatusCode::OK,
        Json(WelcomeMessage {
            msg: "Welcome to AutumnSky!",
        }),
    )
}

pub async fn follow_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<FollowRequest>,
) -> (StatusCode, Json<FollowResponse>) {
    let logic = logic.clone();
    logic.follow(payload.followee, payload.follower).await;
    (
        StatusCode::OK,
        Json(FollowResponse {
            msg: format!("User {} followed {}", payload.follower, payload.followee),
        }),
    )
}

pub async fn unfollow_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<UnfollowRequest>,
) -> (StatusCode, Json<UnfollowResponse>) {
    let logic = logic.clone();
    logic.unfollow(payload.followee, payload.follower).await;
    (
        StatusCode::OK,
        Json(UnfollowResponse {
            msg: format!("User {} unfollowed {}", payload.follower, payload.followee),
        }),
    )
}

pub async fn publish_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<PublishRequest>,
) -> (StatusCode, Json<PublishResponse>) {
    let logic = logic.clone();
    logic.publish(payload.user, 123).await;
    (StatusCode::OK, Json(PublishResponse { status: "Success!" }))
}

pub async fn newsfeed_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<NewsfeedRequest>,
) -> (StatusCode, Json<NewsfeedResponse>) {
    let logic = logic.clone();
    (
        StatusCode::OK,
        Json(NewsfeedResponse {
            status: "ok",
            feed: logic.news_feed(payload.user).await,
        }),
    )
}
