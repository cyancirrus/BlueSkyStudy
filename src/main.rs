mod state;
mod types;
mod routes;
mod handlers;

use std::sync::{Arc}; 
use state::Twitter;
use routes::create_router;
use axum::serve;

const PORT:&'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let mut logic = Arc::new(Twitter::new());
    let router = create_router(logic);
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    {
        serve(listener, router)
        .await
        .unwrap();
    }
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, router).await.unwrap();
}
