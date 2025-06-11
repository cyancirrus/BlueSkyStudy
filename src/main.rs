mod state;
use axum::{
    Router,
    routing::get, 
};
// // use tower::ServiceBuilder;
// use tracing_subscriber;
// use state::TwitApi;
// use state::*;
// // mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"hello world"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
