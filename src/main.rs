mod state;
mod types;
mod routes;
mod handlers;

use state::Twitter;
use routes::create_router;

#[tokio::main]
async fn main() {
    let mut logic = Twitter::new();
    let router = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
