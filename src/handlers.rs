use crate::types::requests::*;
use crate::types::responses::*;
use crate::state::Tweet;
use axum:: {
    extract::Json,
    http::StatusCode,
};



pub async fn welcome_handler() -> (StatusCode, Json<WelcomeMessage>) {
    (
        StatusCode::OK,
        Json(WelcomeMessage{
            msg: "Welcome to AutumnSky!"
        }),
    )
}

pub async fn follow_handler(Json(payload): Json<FollowRequest>) -> (StatusCode, Json<FollowResponse>) {
    (
        StatusCode::OK,
        Json(FollowResponse{
            msg:format!("User {} followed {}", payload.follower, payload.followee)
        })
    )
}

pub async fn unfollow_handler(Json(payload): Json<UnfollowRequest>) -> (StatusCode, Json<UnfollowResponse>) {
    (
    StatusCode::OK,
        Json(UnfollowResponse {
            msg: format!("User {} unfollowed {}", payload.follower, payload.followee),
        })
    )
}

pub async fn publish_handler(Json(payload): Json<PublishRequest>) -> (StatusCode, Json<PublishResponse>) {
    (
        StatusCode::OK,
        Json(PublishResponse {
            status: "Success!",
        })
    )
}

pub async fn newsfeed_handler(Json(payload): Json<NewsfeedRequest>) -> (StatusCode, Json<NewsfeedResponse>) {
    let tweets = vec![
        Tweet::new(1, 42, 123456),
        Tweet::new(2, 99, 123457),
    ];
    (
        StatusCode::OK,
        Json(NewsfeedResponse{
                status:"ok",
                feed:tweets,
        })
    )
}
