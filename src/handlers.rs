use crate::types::requests::*;
use crate::types::responses::*;
use crate::types::aliases::SharedAppState;
// use crate::state::Tweet;
use axum:: {
    extract::{State, Json},
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

pub async fn follow_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<FollowRequest>,
) -> (StatusCode, Json<FollowResponse>) {
    let mut twitter = logic.lock().unwrap();
    twitter.follow(payload.followee, payload.follower);
    (
        StatusCode::OK,
        Json(FollowResponse{
            msg:format!("User {} followed {}", payload.follower, payload.followee)
        })
    )
}

pub async fn unfollow_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<UnfollowRequest>,
) -> (StatusCode, Json<UnfollowResponse>) {
    let mut twitter = logic.lock().unwrap();
    twitter.unfollow(payload.follower, payload.follower);
    // drop(twitter);
    (
    StatusCode::OK,
        Json(UnfollowResponse {
            msg: format!("User {} unfollowed {}", payload.follower, payload.followee),
        })
    )
}

pub async fn publish_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<PublishRequest>,
) -> (StatusCode, Json<PublishResponse>) {
    let mut twitter = logic.lock().unwrap();
    // note need to do
    twitter.publish(payload.user, 123);
    (
        StatusCode::OK,
        Json(PublishResponse {
            status: "Success!",
        })
    )
}

pub async fn newsfeed_handler(
    State(logic): State<SharedAppState>,
    Json(payload): Json<NewsfeedRequest>,
) -> (StatusCode, Json<NewsfeedResponse>) {
    let mut twitter = logic.lock().unwrap();
    twitter.news_feed(payload.user);
    // let tweets = vec![
    //     Tweet::new(1, 42, 123456),
    //     Tweet::new(2, 99, 123457),
    // ];
    (
        StatusCode::OK,
        Json(NewsfeedResponse{
                status:"ok",
                feed: twitter.news_feed(payload.user),
        })
    )
}
