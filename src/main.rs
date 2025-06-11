mod state;
mod types;
mod routes;
mod handlers;

use std::sync::{Arc, Mutex}; 
use state::Twitter;
use routes::create_router;
use axum::serve;

const PORT:&'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let logic = Arc::new(Mutex::new(Twitter::new()));
    let router = create_router(logic);
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    {
        serve(listener, router)
        .await
        .unwrap();
    }
}
