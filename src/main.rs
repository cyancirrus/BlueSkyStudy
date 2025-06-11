mod state;
// mod contracts;
use serde::{
    Serialize,
    Deserialize,
};
use axum:: {
    extract::Json,
    http::StatusCode,
};
use axum::{
    Router,
    routing::get, 
    routing::post, 
};
use state::*;
// // use tower::ServiceBuilder;
// use tracing_subscriber;
// use state::TwitApi;
// use state::*;
// // mod routes;


// Requests
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
    pub reason:String,
}

// Responses
#[derive(Debug, Serialize)]
pub struct WelcomeMessage {
    pub msg:&'static str,
}

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    msg: String, 
}

#[derive(Debug, Serialize)]
pub struct UnfollowResponse {
    msg: String, 
}

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub status:&'static str
}

#[derive(Debug, Serialize)]
pub struct NewsfeedResponse {
    pub status:&'static str,
    pub feed: Vec<Tweet>,
}

async fn welcome_handler() -> (StatusCode, Json<WelcomeMessage>) {
    (
        StatusCode::OK,
        Json(WelcomeMessage{
            msg: "Welcome to AutumnSky!"
        }),
    )
}

async fn follow_handler(Json(payload): Json<FollowRequest>) -> (StatusCode, Json<FollowResponse>) {
    (
        StatusCode::OK,
        Json(FollowResponse{
            msg:format!("User {} followed {}", payload.follower, payload.followee)
        })
    )
}

async fn unfollow_handler(Json(payload): Json<UnfollowRequest>) -> (StatusCode, Json<UnfollowResponse>) {
    (
    StatusCode::OK,
        Json(UnfollowResponse {
            msg: format!("User {} unfollowed {}", payload.follower, payload.followee),
        })
    )
}

async fn publish_handler(Json(payload): Json<PublishRequest>) -> (StatusCode, Json<PublishResponse>) {
    (
        StatusCode::OK,
        Json(PublishResponse {
            status: "Success!",
        })
    )
}

async fn newsfeed_handler(Json(payload): Json<NewsfeedRequest>) -> (StatusCode, Json<NewsfeedResponse>) {
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


#[tokio::main]
async fn main() {
    let mut logic = state::Twitter::new();
    let endpoints = {
        Router::new()
        .route("/", get(welcome_handler))
        .route("/follow", post(follow_handler))
        .route("/unfollow", post(unfollow_handler))
        .route("/publish", post(publish_handler))
        .route("/newsfeed", post(newsfeed_handler))
    };


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, endpoints).await.unwrap();
}
