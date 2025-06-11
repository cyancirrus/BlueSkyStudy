mod state;
use axum::{
    Router,
    routing::get, 
    routing::post, 
};
// // use tower::ServiceBuilder;
// use tracing_subscriber;
// use state::TwitApi;
// use state::*;
// // mod routes;

#[tokio::main]
async fn main() {
    let mut logic = state::Twitter::new();
    let endpoints = Router::new()
        .route("/", get(|| async {"hello world"}))
        .route("/subscribe", post(|| async {subscribe().await}))
        .route("/publish", post(|| async {"publish"}))
        .route("/newsfeed", post(|| async {"newsfeed"}))
        .route("/unsubscribe", post(|| async {"unsubscribing"}));



    async fn subscribe() -> String {
        format!("Async i am subscribing")
    }




    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, endpoints).await.unwrap();
}
