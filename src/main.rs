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
#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    pub follower: usize,
    pub followee: usize,
}

#[derive(Debug, Deserialize)]
pub struct NewsfeedRequest {
    pub user: UserId,
}

#[derive(Debug, Deserialize)]
pub struct UnfollowRequest {
    pub follower: usize,
    pub followee: usize,
    pub reason:String,
}

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    message: String, 
}

#[derive(Debug, Serialize)]
pub struct NewsfeedResponse {
    pub status:&'static str,
    pub feed: Vec<Tweet>,
}

async fn follow_handler(Json(payload): Json<FollowRequest>) -> (StatusCode, Json<FollowResponse>) {
    (
        StatusCode::OK,
        Json(FollowResponse{
            message:format!("User {} followed {}", payload.follower, payload.followee)
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

async fn unfollow_handler(Json(payload): Json<UnfollowRequest>) -> StatusCode {
    println!("User {} unfollowed {}", payload.follower, payload.followee);
    StatusCode::OK
}

async fn landing_handler(Json(payload): Json<UnfollowRequest>) -> StatusCode {
    println!("Welcome to AutumnSky!");
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let mut logic = state::Twitter::new();
    let endpoints = {
        Router::new()
        .route("/", get(|| async {"hello world"}))
        .route("/follow", post(follow_handler))
        .route("/unfollow", post(unfollow_handler))
        .route("/publish", post(|| async {"publish"}))
        .route("/newsfeed", post(newsfeed_handler))
    };


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, endpoints).await.unwrap();
}
